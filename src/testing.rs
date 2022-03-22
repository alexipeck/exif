#[cfg(test)]
mod tests {
    use std::path::Path;

    use crate::exif::Exif;

    #[test]
    fn test_pull_exif_data() {
        match Exif::new(Path::new(r"C:\Users\Me\Desktop\mapping_tool\target\release\source_one\DJI_0013.JPG")) {
            Ok(exif) => {
                for (tag, value) in exif.attributes.iter() {
                    println!("{}:{}", tag, value);
                }
            },
            Err(err) => {
                println!("{}", err);
            },
        };
    }
}