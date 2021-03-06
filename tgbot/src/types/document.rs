use crate::types::{photo_size::PhotoSize, primitive::Integer};
use serde::Deserialize;

/// General file (as opposed to photos, voice messages and audio files)
#[derive(Clone, Debug, Deserialize)]
pub struct Document {
    /// Unique file identifier
    pub file_id: String,
    /// Document thumbnail as defined by sender
    pub thumb: Option<PhotoSize>,
    /// Original filename as defined by sender
    pub file_name: Option<String>,
    /// MIME type of the file as defined by sender
    pub mime_type: Option<String>,
    /// File size
    pub file_size: Option<Integer>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn deserialize_full() {
        let data: Document = serde_json::from_value(serde_json::json!({
            "file_id": "SSSxmmmsmsIIsooofiiiiaiiaIII_XLA",
            "thumb": {
                "file_id": "AdddddUuUUUUccccUUmm_PPP",
                "width": 24,
                "height": 24,
                "file_size": 12324
            },
            "file_name": "Test file name",
            "mime_type": "image/jpeg",
            "file_size": 1234
        }))
        .unwrap();

        assert_eq!(data.file_id, "SSSxmmmsmsIIsooofiiiiaiiaIII_XLA");

        let thumb = data.thumb.unwrap();
        assert_eq!(thumb.file_id, "AdddddUuUUUUccccUUmm_PPP");
        assert_eq!(thumb.width, 24);
        assert_eq!(thumb.height, 24);
        assert_eq!(thumb.file_size.unwrap(), 12324);

        assert_eq!(data.file_name.unwrap(), "Test file name");
        assert_eq!(data.mime_type.unwrap(), "image/jpeg");
        assert_eq!(data.file_size.unwrap(), 1234);
    }

    #[test]
    fn deserialize_partial() {
        let data: Document = serde_json::from_value(serde_json::json!({
            "file_id": "SSSxmmmsmsIIsooofiiiiaiiaIII_XLA"
        }))
        .unwrap();
        assert_eq!(data.file_id, "SSSxmmmsmsIIsooofiiiiaiiaIII_XLA");
        assert!(data.file_name.is_none());
        assert!(data.thumb.is_none());
        assert!(data.mime_type.is_none());
        assert!(data.file_size.is_none());
    }
}
