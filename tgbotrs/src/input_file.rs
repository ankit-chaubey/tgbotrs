use bytes::Bytes;
use serde::{Deserialize, Serialize, Serializer};

/// Represents a file to be sent.
///
/// Can be one of:
/// - A `file_id` string (reference an existing file on Telegram's servers)
/// - A URL string (Telegram downloads the file from the URL)
/// - Raw bytes with a filename (uploaded directly via multipart)
#[derive(Debug, Clone)]
pub enum InputFile {
    /// A file already uploaded to Telegram, referenced by `file_id`.
    FileId(String),
    /// A URL that Telegram will download.
    Url(String),
    /// Raw bytes with a filename to upload via multipart.
    Memory { filename: String, data: Bytes },
}

impl InputFile {
    /// Create an InputFile from a file_id string.
    pub fn file_id(id: impl Into<String>) -> Self {
        InputFile::FileId(id.into())
    }

    /// Create an InputFile from a URL.
    pub fn url(url: impl Into<String>) -> Self {
        InputFile::Url(url.into())
    }

    /// Create an InputFile from raw bytes.
    pub fn memory(filename: impl Into<String>, data: impl Into<Bytes>) -> Self {
        InputFile::Memory {
            filename: filename.into(),
            data: data.into(),
        }
    }
}

// InputFile serializes to its string representation (file_id or URL).
// Memory files are handled separately when building multipart requests.
impl Serialize for InputFile {
    fn serialize<S: Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            InputFile::FileId(id) => serializer.serialize_str(id),
            InputFile::Url(url) => serializer.serialize_str(url),
            InputFile::Memory { filename, .. } => {
                serializer.serialize_str(&format!("attach://{}", filename))
            }
        }
    }
}

impl<'de> Deserialize<'de> for InputFile {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let s = String::deserialize(deserializer)?;
        if s.starts_with("http://") || s.starts_with("https://") {
            Ok(InputFile::Url(s))
        } else {
            Ok(InputFile::FileId(s))
        }
    }
}

impl From<String> for InputFile {
    fn from(s: String) -> Self {
        InputFile::FileId(s)
    }
}
impl From<&str> for InputFile {
    fn from(s: &str) -> Self {
        InputFile::FileId(s.to_string())
    }
}

// ─────────────────────────────────────────────────
// InputFileOrString
// ─────────────────────────────────────────────────

/// A field that can be either an InputFile or a String (file_id / URL).
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(untagged)]
pub enum InputFileOrString {
    File(InputFile),
    String(String),
}

impl From<InputFile> for InputFileOrString {
    fn from(f: InputFile) -> Self {
        InputFileOrString::File(f)
    }
}
impl From<String> for InputFileOrString {
    fn from(s: String) -> Self {
        InputFileOrString::String(s)
    }
}
impl From<&str> for InputFileOrString {
    fn from(s: &str) -> Self {
        InputFileOrString::String(s.to_string())
    }
}
