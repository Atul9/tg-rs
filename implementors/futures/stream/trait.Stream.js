(function() {var implementors = {};
implementors["common_multipart_rfc7578"] = [{text:"impl&lt;'a&gt; <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"common_multipart_rfc7578/client/multipart/struct.Body.html\" title=\"struct common_multipart_rfc7578::client::multipart::Body\">Body</a>&lt;'a&gt;",synthetic:false,types:["common_multipart_rfc7578::client_::Body"]},];
implementors["futures"] = [];
implementors["h2"] = [{text:"impl <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"h2/client/struct.PushPromises.html\" title=\"struct h2::client::PushPromises\">PushPromises</a>",synthetic:false,types:["h2::client::PushPromises"]},{text:"impl&lt;T, B&gt; <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"h2/server/struct.Connection.html\" title=\"struct h2::server::Connection\">Connection</a>&lt;T, B&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"tokio_io/async_read/trait.AsyncRead.html\" title=\"trait tokio_io::async_read::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"tokio_io/async_write/trait.AsyncWrite.html\" title=\"trait tokio_io::async_write::AsyncWrite\">AsyncWrite</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;B: <a class=\"trait\" href=\"bytes/buf/into_buf/trait.IntoBuf.html\" title=\"trait bytes::buf::into_buf::IntoBuf\">IntoBuf</a>,<br>&nbsp;&nbsp;&nbsp;&nbsp;B::<a class=\"type\" href=\"bytes/buf/into_buf/trait.IntoBuf.html#associatedtype.Buf\" title=\"type bytes::buf::into_buf::IntoBuf::Buf\">Buf</a>: 'static,&nbsp;</span>",synthetic:false,types:["h2::server::Connection"]},{text:"impl <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"h2/struct.RecvStream.html\" title=\"struct h2::RecvStream\">RecvStream</a>",synthetic:false,types:["h2::share::RecvStream"]},];
implementors["tgbot"] = [{text:"impl <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tgbot/struct.UpdatesStream.html\" title=\"struct tgbot::UpdatesStream\">UpdatesStream</a>",synthetic:false,types:["tgbot::handler::poll::UpdatesStream"]},];
implementors["tokio_fs"] = [{text:"impl <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_fs/struct.ReadDir.html\" title=\"struct tokio_fs::ReadDir\">ReadDir</a>",synthetic:false,types:["tokio_fs::read_dir::ReadDir"]},];
implementors["tokio_io"] = [{text:"impl&lt;A&gt; <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_io/io/struct.Lines.html\" title=\"struct tokio_io::io::Lines\">Lines</a>&lt;A&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;A: <a class=\"trait\" href=\"tokio_io/trait.AsyncRead.html\" title=\"trait tokio_io::AsyncRead\">AsyncRead</a> + <a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/std/io/trait.BufRead.html\" title=\"trait std::io::BufRead\">BufRead</a>,&nbsp;</span>",synthetic:false,types:["tokio_io::lines::Lines"]},];
implementors["tokio_sync"] = [{text:"impl&lt;T&gt; <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_sync/mpsc/struct.Receiver.html\" title=\"struct tokio_sync::mpsc::Receiver\">Receiver</a>&lt;T&gt;",synthetic:false,types:["tokio_sync::mpsc::bounded::Receiver"]},{text:"impl&lt;T&gt; <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_sync/mpsc/struct.UnboundedReceiver.html\" title=\"struct tokio_sync::mpsc::UnboundedReceiver\">UnboundedReceiver</a>&lt;T&gt;",synthetic:false,types:["tokio_sync::mpsc::unbounded::UnboundedReceiver"]},{text:"impl&lt;T:&nbsp;<a class=\"trait\" href=\"https://doc.rust-lang.org/nightly/core/clone/trait.Clone.html\" title=\"trait core::clone::Clone\">Clone</a>&gt; <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_sync/watch/struct.Receiver.html\" title=\"struct tokio_sync::watch::Receiver\">Receiver</a>&lt;T&gt;",synthetic:false,types:["tokio_sync::watch::Receiver"]},];
implementors["tokio_tcp"] = [{text:"impl <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_tcp/struct.Incoming.html\" title=\"struct tokio_tcp::Incoming\">Incoming</a>",synthetic:false,types:["tokio_tcp::incoming::Incoming"]},];
implementors["tokio_timer"] = [{text:"impl&lt;T&gt; <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_timer/delay_queue/struct.DelayQueue.html\" title=\"struct tokio_timer::delay_queue::DelayQueue\">DelayQueue</a>&lt;T&gt;",synthetic:false,types:["tokio_timer::delay_queue::DelayQueue"]},{text:"impl&lt;T:&nbsp;<a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a>&gt; <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_timer/throttle/struct.Throttle.html\" title=\"struct tokio_timer::throttle::Throttle\">Throttle</a>&lt;T&gt;",synthetic:false,types:["tokio_timer::throttle::Throttle"]},{text:"impl&lt;T&gt; <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_timer/timeout/struct.Timeout.html\" title=\"struct tokio_timer::timeout::Timeout\">Timeout</a>&lt;T&gt; <span class=\"where fmt-newline\">where<br>&nbsp;&nbsp;&nbsp;&nbsp;T: <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a>,&nbsp;</span>",synthetic:false,types:["tokio_timer::timeout::Timeout"]},{text:"impl <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_timer/struct.Interval.html\" title=\"struct tokio_timer::Interval\">Interval</a>",synthetic:false,types:["tokio_timer::interval::Interval"]},];
implementors["tokio_udp"] = [{text:"impl&lt;C:&nbsp;<a class=\"trait\" href=\"tokio_io/codec/decoder/trait.Decoder.html\" title=\"trait tokio_io::codec::decoder::Decoder\">Decoder</a>&gt; <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_udp/struct.UdpFramed.html\" title=\"struct tokio_udp::UdpFramed\">UdpFramed</a>&lt;C&gt;",synthetic:false,types:["tokio_udp::frame::UdpFramed"]},];
implementors["tokio_uds"] = [{text:"impl&lt;A, C:&nbsp;<a class=\"trait\" href=\"tokio_io/codec/decoder/trait.Decoder.html\" title=\"trait tokio_io::codec::decoder::Decoder\">Decoder</a>&gt; <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_uds/struct.UnixDatagramFramed.html\" title=\"struct tokio_uds::UnixDatagramFramed\">UnixDatagramFramed</a>&lt;A, C&gt;",synthetic:false,types:["tokio_uds::frame::UnixDatagramFramed"]},{text:"impl <a class=\"trait\" href=\"futures/stream/trait.Stream.html\" title=\"trait futures::stream::Stream\">Stream</a> for <a class=\"struct\" href=\"tokio_uds/struct.Incoming.html\" title=\"struct tokio_uds::Incoming\">Incoming</a>",synthetic:false,types:["tokio_uds::incoming::Incoming"]},];

            if (window.register_implementors) {
                window.register_implementors(implementors);
            } else {
                window.pending_implementors = implementors;
            }
        
})()
