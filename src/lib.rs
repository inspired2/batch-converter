use std::fs::DirEntry;

//processing images:
extern crate image;
#[derive(Debug, PartialEq)]
pub enum ImageFormat {
    JPEG,
    WEBP,
    PNG,
    HEIC,
    Other
}
pub struct Image {
    path: String,
    format: ImageFormat
}

impl Image {
    fn new(path: String) -> Self {
        let format = Image::derive_format(&path);
        Self {
            path,
            format
        }
    }
    fn derive_format(path: &String) -> ImageFormat {
        use self::ImageFormat::*;
        let file_extension = std::path::Path::new(&path).extension().expect("could not parse file extension");
        match file_extension.to_str().expect("unable to stringify file extension") {
            "jpeg" | "jpg"=> JPEG,
            "heic" | "heif" => HEIC,
            "png" => PNG,
            "webp" => WEBP,
            _ => Other
        }
    }
    fn convert_to_jpeg(&self) -> Option<()> {
       unimplemented!()
   }
}

pub fn process_all(dir_path: &str) -> Result<(), std::io::Error> {
    let entries = std::fs::read_dir(dir_path).unwrap();
    let correct_entries = entries.map(Result::unwrap).collect::<DirEntry>();
    //read all entries in dir
    //if entry is a processable image other than jpeg => convert it to jpeg
    Ok(())
}
fn convert() {
    //
}



#[cfg(test)]
mod test {
    use Image;
    use ImageFormat;
    #[test]
    fn test_extensions() {
        let image = Image::new("/dir/file.jpeg".to_string());
        assert_eq!(image.format, ImageFormat::JPEG);
    }
}