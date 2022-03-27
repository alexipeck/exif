use std::{
    collections::{HashMap, HashSet},
    fmt,
    path::Path,
    process::{Command, Stdio},
    sync::Arc,
};

type List = Arc<HashSet<String>>;

pub enum ExifError {
    FileNotFound(String),
    TagError(String),
    ValueError(String),
    ExifToolError(String),
    FromUtf8Error(String),
    UncontrolledError(String),
}

impl fmt::Display for ExifError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::FileNotFound(error_message) => format!("FileNotFound: {}", error_message),
                Self::TagError(error_message) => format!("TagError: {}", error_message),
                Self::ValueError(error_message) => format!("ValueError: {}", error_message),
                Self::ExifToolError(error_message) => format!("ExifToolError: {}", error_message),
                Self::FromUtf8Error(error_message) => format!("FromUtf8Error: {}", error_message),
                Self::UncontrolledError(error_message) =>
                    format!("UncontrolledError: {}", error_message),
            }
        )
    }
}

pub fn create_list_from_vec(tag_list: Vec<&str>) -> List {
    let mut list: HashSet<String> = HashSet::new();
    for element in tag_list {
        list.insert(element.to_string());
    }
    Arc::new(list)
}

pub fn exiftool_available() -> bool {
    return Command::new("exiftool")
        .stdout(Stdio::null())
        .stderr(Stdio::null())
        .spawn()
        .is_ok();
}

pub enum Mode {
    All,
    Whitelist(List),
    Blacklist(List),
}

pub struct Exif {
    pub attributes: HashMap<String, String>,
}

impl Exif {
    fn default() -> Self {
        Self {
            attributes: HashMap::new(),
        }
    }
    pub fn new(file_path: &Path, mode: Mode) -> Result<Self, ExifError> {
        let mut exif = Self::default();
        let child = match Command::new("exiftool")
            .arg(file_path)
            .stdout(Stdio::piped())
            .stderr(Stdio::piped())
            .spawn()
        {
            Ok(child) => child,
            Err(err) => {
                return Err(ExifError::ExifToolError(err.to_string()));
            }
        };

        let output = match child.wait_with_output() {
            Ok(output) => output,
            Err(err) => {
                return Err(ExifError::ExifToolError(err.to_string()));
            }
        };

        let stdout_output = match String::from_utf8(output.stdout) {
            Ok(output) => output,
            Err(err) => {
                return Err(ExifError::FromUtf8Error(err.to_string()));
            }
        };

        let stderr_output = match String::from_utf8(output.stderr) {
            Ok(output) => output,
            Err(err) => {
                return Err(ExifError::FromUtf8Error(err.to_string()));
            }
        };

        if stderr_output.contains("Error: File not found -") {
            return Err(ExifError::FileNotFound(stderr_output.trim().to_string()));
        }

        let output = stdout_output.split('\n');
        for line in output {
            let line = String::from(line);
            let line_split = line.split(':');
            let tag = match line_split.clone().next() {
                Some(tag) => {
                    let mut tag = tag.trim().to_string();
                    tag.retain(|c| !c.is_whitespace());
                    tag
                }
                None => {
                    return Err(ExifError::TagError(
                        "Error getting tag from Exif data".to_string(),
                    ));
                }
            };
            if tag.is_empty() {
                continue;
            }

            match &mode {
                Mode::All => {}
                Mode::Whitelist(list) => {
                    if list.contains(&tag) {
                        break;
                    } else {
                        continue;
                    }
                }
                Mode::Blacklist(list) => {
                    if !list.contains(&tag) {
                        break;
                    } else {
                        continue;
                    }
                }
            }

            let value = match line_split.last() {
                Some(value) => value.trim().to_string(),
                None => {
                    return Err(ExifError::TagError(
                        "Error getting value from Exif data".to_string(),
                    ));
                }
            };
            exif.attributes.insert(tag, value);
        }
        Ok(exif)
    }
}
