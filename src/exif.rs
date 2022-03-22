use std::{
    collections::{HashMap, HashSet},
    path::Path,
    process::{exit, Command, Stdio},
    sync::Arc,
    thread,
    time::Duration,
};

pub const EXIT_DELAY: Duration = Duration::from_millis(250);

pub struct Exif {
    pub attributes: HashMap<String, String>,
}

impl Exif {
    fn default() -> Self {
        if Command::new("exiftool")
            .stdout(Stdio::null())
            .stderr(Stdio::null())
            .spawn()
            .is_err()
        {
            println!("Error: Missing dependency ffmpeg.");
            thread::sleep(EXIT_DELAY);
            exit(1);
        }
        Self {
            attributes: HashMap::new(),
        }
    }

    /* pub fn new_whitelisted(_file_path: &Path, _whitelisted_tags: Arc<HashSet<String>>) -> Self {
        let exif = Self::default();
        exif
    } */

    /* pub fn new_blacklisted(_file_path: &Path, _blacklisted_tags: Arc<HashSet<String>>) -> Self {
        let exif = Self::default();
        exif
    } */

    pub fn new(file_path: &Path) -> Result<Self, String> {
        let mut exif = Self::default();
        let child = match Command::new("exiftool")
            .arg(file_path)
            .stdout(Stdio::piped())
            .spawn() {
                Ok(child) => child,
                Err(err) => {
                    return Err(err.to_string());
                },
            };

        let output = match child.wait_with_output() {
            Ok(output) => output,
            Err(err) => {
                return Err(err.to_string());
            },
        };
        
        let output = match String::from_utf8(output.stdout) {
            Ok(output) => output,
            Err(err) => {
                return Err(err.to_string());
            },
        };
        let output = output.split('\n');
        for line in output {
            let line = String::from(line);
            let line_split = line.split(':');
            let tag = match line_split.clone().next() {
                Some(tag) => {
                    let mut tag = tag.trim().to_string();
                    tag.retain(|c| !c.is_whitespace());
                    tag
                },
                None => {
                    return Err("Error getting tag from Exif data".to_string());
                },
            };
            if tag == "" {
                continue;
            }
            let value = match line_split.last() {
                Some(value) => value.trim().to_string(),
                None => {
                    return Err("Error getting value from Exif data".to_string());
                },
            };
            exif.attributes.insert(tag, value);
        }
        Ok(exif)
    }
}