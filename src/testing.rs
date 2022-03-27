#[cfg(test)]
mod tests {
    use crate::exif::{create_list_from_vec, exiftool_available, Exif, Mode};
    use std::path::Path;

    pub const TEST_FILE_PATH: &str =
        r"C:\Users\Alexi Peck\Desktop\mapping_tool\target\release\source_one\DJI_0013.JPG";

    #[test]
    fn test_exiftool_available() {
        if !exiftool_available() {
            panic!("Exiftool not available for execution.");
        }
    }

    #[test]
    fn test_pull_exif_data_all() {
        match Exif::new(Path::new(TEST_FILE_PATH), Mode::All) {
            Ok(exif) => {
                for (tag, value) in exif.attributes.iter() {
                    println!("{}:{}", tag, value);
                }
            }
            Err(err) => {
                panic!("{}", err);
            }
        };
    }

    #[test]
    fn test_pull_exif_data_whitelist() {
        let whitelist = create_list_from_vec(vec![
            "GPSLatitude",
            "GPSLongitude",
            "GPSAltitude",
            "ExifImageWidth",
            "ExifImageHeight",
            "FlightYawDegree",
            "AbsoluteAltitude",
            "RelativeAltitude",
            "FieldOfView",
            "FocalLength",
        ]);

        match Exif::new(Path::new(TEST_FILE_PATH), Mode::Whitelist(whitelist)) {
            Ok(exif) => {
                for (tag, value) in exif.attributes.iter() {
                    println!("{}:{}", tag, value);
                }
            }
            Err(err) => {
                panic!("{}", err);
            }
        };
    }

    #[test]
    fn test_pull_exif_data_blacklist() {
        let blacklist = create_list_from_vec(vec![
            "SerialNumber",
            "FileModificationDate/Time",
            "DigitalZoomRatio",
            "XPComment",
            "XPKeywords",
        ]);

        match Exif::new(Path::new(TEST_FILE_PATH), Mode::Blacklist(blacklist)) {
            Ok(exif) => {
                for (tag, value) in exif.attributes.iter() {
                    println!("{}:{}", tag, value);
                }
            }
            Err(err) => {
                panic!("{}", err);
            }
        };
    }
}
