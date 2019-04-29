use crate::{
    executor::Executor,
    request::{Request, RequestBody, RequestMethod},
};
use failure::Error;
use futures01::Stream;
use futures03::{compat::Future01CompatExt, Future};
use hyper::{
    client::{connect::Connect, Client, HttpConnector},
    Body, Chunk, Request as HttpRequest, Response,
};
use hyper_multipart_rfc7578::client::multipart::{Body as MultipartBody, Form as MultipartForm};
use hyper_proxy::{Intercept as HttpProxyIntercept, Proxy as HttpProxy, ProxyConnector as HttpProxyConnector};
use hyper_socks2::{Auth as SocksAuth, Proxy as SocksProxy};
use hyper_tls::HttpsConnector;
use log::{debug, log_enabled, Level::Debug};
use std::{net::SocketAddr, pin::Pin, sync::Arc};
use typed_headers::Credentials as HttpProxyCredentials;
use url::{percent_encoding::percent_decode, Url};

const DEFAULT_HTTPS_DNS_WORKER_THREADS: usize = 1;

struct HyperExecutor<C> {
    client: Arc<Client<C>>,
}

impl<C> HyperExecutor<C> {
    fn new(client: Client<C>) -> Self {
        HyperExecutor {
            client: Arc::new(client),
        }
    }
}

impl<C: Connect + 'static> Executor for HyperExecutor<C> {
    fn execute(&self, req: Request) -> Pin<Box<Future<Output = Result<Vec<u8>, Error>> + Send>> {
        let client = self.client.clone();
        Box::pin(
            async move {
                let mut builder = match req.method {
                    RequestMethod::Get => HttpRequest::get(req.url),
                    RequestMethod::Post => HttpRequest::post(req.url),
                };
                let req = match req.body {
                    RequestBody::Form(form) => {
                        MultipartForm::from(form).set_body_convert::<Body, MultipartBody>(&mut builder)
                    }
                    RequestBody::Json(data) => {
                        if log_enabled!(Debug) {
                            debug!("Post JSON data: {}", String::from_utf8_lossy(&data));
                        }
                        builder.header("Content-Type", "application/json");
                        builder.body(data.into())
                    }
                    RequestBody::Empty => builder.body(Body::empty()),
                }?;
                let resp: Response<Body> = await!(client.request(req).compat())?;
                let body: Chunk = await!(resp.into_body().concat2().compat())?;
                let body = body.to_vec();
                if log_enabled!(Debug) {
                    debug!("Got response: {}", String::from_utf8_lossy(&body));
                }
                Ok(body)
            },
        )
    }
}

fn https_connector() -> Result<HttpsConnector<HttpConnector>, Error> {
    Ok(HttpsConnector::new(DEFAULT_HTTPS_DNS_WORKER_THREADS)?)
}

pub(crate) fn default_executor() -> Result<Box<Executor>, Error> {
    let connector = https_connector()?;
    let client = Client::builder().build(connector);
    Ok(Box::new(HyperExecutor::new(client)))
}

fn socks_proxy_executor(proxy: SocksProxy<SocketAddr>) -> Result<Box<Executor>, Error> {
    let connector = proxy.with_tls()?;
    let client = Client::builder().build(connector);
    Ok(Box::new(HyperExecutor::new(client)))
}

fn http_proxy_executor(proxy: HttpProxy) -> Result<Box<Executor>, Error> {
    let connector = https_connector()?;
    let proxy_connector = HttpProxyConnector::from_proxy(connector, proxy)?;
    let client = Client::builder().build(proxy_connector);
    Ok(Box::new(HyperExecutor::new(client)))
}

#[derive(Debug, failure::Fail)]
#[fail(display = "Unexpected proxy: {}", _0)]
struct UnexpectedProxyError(String);

pub(crate) fn proxy_executor(dsn: &str) -> Result<Box<Executor>, Error> {
    macro_rules! unexpected_proxy {
        () => {
            return Err(UnexpectedProxyError(dsn.to_string()).into());
        };
    }
    let parsed_dsn = Url::parse(dsn)?;
    let host: SocketAddr = match (parsed_dsn.host_str(), parsed_dsn.port()) {
        (Some(host), Some(port)) => format!("{}:{}", host, port).parse()?,
        _ => unexpected_proxy!(),
    };
    match parsed_dsn.scheme() {
        "http" | "https" => {
            let mut proxy = HttpProxy::new(HttpProxyIntercept::All, dsn.parse()?);
            if let Some(password) = parsed_dsn.password() {
                proxy.set_authorization(HttpProxyCredentials::basic(parsed_dsn.username(), password)?);
            }
            http_proxy_executor(proxy)
        }
        "socks4" => socks_proxy_executor(SocksProxy::Socks4 {
            addrs: host,
            user_id: parsed_dsn.username().to_string(),
        }),
        "socks5" => socks_proxy_executor(SocksProxy::Socks5 {
            addrs: host,
            auth: parsed_dsn.password().map(|password| SocksAuth {
                user: parsed_dsn.username().to_string(),
                pass: percent_decode(password.as_bytes()).decode_utf8_lossy().to_string(),
            }),
        }),
        _ => unexpected_proxy!(),
    }
}
