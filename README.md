Exif reader.
Pull all exif tags and values from a file, requires exiftool to be installed.
There are 3 modes of importing, All, Whitelist and Blacklist.

```rust
pub const TEST_FILE_PATH: &str = r"PATH_TO_FILE";
pub fn main() {
    if !exiftool_available() {
        //handle error
        panic!("Exiftool not available for execution.");
    }
    
    //pull all exif tags/values
    match Exif::new(Path::new(TEST_FILE_PATH), Mode::All) {
        Ok(exif) => {
            for (tag, value) in exif.attributes.iter() {
                println!("{}:{}", tag, value);
            }
        }
        Err(err) => {
            //handle error
            panic!("{}", err);
        }
    };
    
    //pull exif tags/values filtered by whitelist
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
            //handle error
            panic!("{}", err);
        }
    };
    
    //pull exif tags/values filtered by blacklist
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
            //handle error
            panic!("{}", err);
        }
    };
}
```