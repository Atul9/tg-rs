use crate::types::{photo_size::PhotoSize, primitive::Integer};
use serde::Deserialize;

/// Video message
#[derive(Clone, Debug, Deserialize)]
pub struct VideoNote {
    /// Unique identifier for this file
    pub file_id: String,
    /// Video width and height
    pub length: Integer,
    ///  Duration of the video in seconds
    pub duration: Integer,
    /// Video thumbnail
    pub thumb: Option<PhotoSize>,
    /// File size
    pub file_size: Option<Integer>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_full() {
        let data: VideoNote = serde_json::from_value(serde_json::json!({
            "file_id": "video note file id",
            "length": 124,
            "duration": 1234,
            "thumb": {
                "file_id": "AdddddUuUUUUccccUUmm_PPP",
                "width": 24,
                "height": 24,
                "file_size": 12324
            },
            "file_size": 12345
        }))
        .unwrap();

        assert_eq!(data.file_id, "video note file id");
        assert_eq!(data.length, 124);
        assert_eq!(data.duration, 1234);

        let thumb = data.thumb.unwrap();
        assert_eq!(thumb.file_id, "AdddddUuUUUUccccUUmm_PPP");
        assert_eq!(thumb.width, 24);
        assert_eq!(thumb.height, 24);
        assert_eq!(thumb.file_size.unwrap(), 12324);

        assert_eq!(data.file_size.unwrap(), 12345);
    }

    #[test]
    fn deserialize_partial() {
        let data: VideoNote = serde_json::from_value(serde_json::json!({
            "file_id": "video note file id",
            "length": 124,
            "duration": 1234
        }))
        .unwrap();

        assert_eq!(data.file_id, "video note file id");
        assert_eq!(data.length, 124);
        assert_eq!(data.duration, 1234);
        assert!(data.thumb.is_none());
        assert!(data.file_size.is_none());
    }
}
