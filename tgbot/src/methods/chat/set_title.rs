use crate::{methods::Method, request::RequestBuilder, types::ChatId};
use failure::Error;
use serde::Serialize;

/// Change the title of a chat
///
/// Titles can't be changed for private chats
/// The bot must be an administrator in the chat for this to work
/// and must have the appropriate admin rights
///
/// Note: In regular groups (non-supergroups), this method will only work
/// if the ‘All Members Are Admins’ setting is off in the target group
#[derive(Clone, Debug, Serialize)]
pub struct SetChatTitle {
    chat_id: ChatId,
    title: String,
}

impl SetChatTitle {
    /// Creates a new SetChatTitle
    ///
    /// # Arguments
    ///
    /// * chat_id - Unique identifier for the target chat
    /// * title - New chat title, 1-255 characters
    pub fn new<C: Into<ChatId>, S: Into<String>>(chat_id: C, title: S) -> Self {
        SetChatTitle {
            chat_id: chat_id.into(),
            title: title.into(),
        }
    }
}

impl Method for SetChatTitle {
    type Response = bool;

    fn into_request(self) -> Result<RequestBuilder, Error> {
        RequestBuilder::json("setChatTitle", &self)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::request::{RequestBody, RequestMethod};
    use serde_json::Value;

    #[test]
    fn set_chat_title() {
        let request = SetChatTitle::new(1, "title")
            .into_request()
            .unwrap()
            .build("base-url", "token");
        assert_eq!(request.method, RequestMethod::Post);
        assert_eq!(request.url, "base-url/bottoken/setChatTitle");
        if let RequestBody::Json(data) = request.body {
            let data: Value = serde_json::from_slice(&data).unwrap();
            assert_eq!(data["chat_id"], 1);
            assert_eq!(data["title"], "title");
        } else {
            panic!("Unexpected request body: {:?}", request.body);
        }
    }
}
