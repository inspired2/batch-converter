#![feature(is_symlink)]
extern crate image;
extern crate rayon;

use std::{fs::DirEntry, path::Path};
use rayon::prelude::*;
type CustomError = Box<dyn std::error::Error>;


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
    //format: ImageFormat
}

impl Image {
    fn new(path: String) -> Self {
        //let format = Image::derive_format(&path);
        Self {
            path,
            //format
        }
    }
    fn derive_format(path: &String) -> ImageFormat {
        use self::ImageFormat::*;
        let file_extension = std::path::Path::new(&path).extension().expect("could not parse file extension");
        match file_extension.to_str().expect("unable to stringify file extension") {
            "jpeg" | "jpg" => JPEG,
            "heic" | "heif" => HEIC,
            "png" => PNG,
            "webp" => WEBP,
            _ => Other
        }
    }
    fn convert_to_jpeg(self) -> Result<(), CustomError> {
        //here any image is PNG
        let dyn_image = image::open(&self.path)?;
        let mut path_buf = Path::new(&self.path).to_path_buf();
        path_buf.set_extension("jpeg").to_string();
        dyn_image.save_with_format( &path_buf, image::ImageFormat::Jpeg)?;
        std::fs::remove_file(self.path)?;

        Ok(())
    }

}


pub fn process_all(path: String) -> Result<(), CustomError> {
    println!("{}", path);
    let files = collect_all_files(&path)?;
    let non_jpeg_images: Vec<Image> = files.into_iter().filter(|p| is_manageable(p)).map(Image::new).collect();
    non_jpeg_images.into_par_iter().for_each(move |img| { img.convert_to_jpeg().expect("error while converting image to jpeg");});
    Ok(())
}


pub fn collect_all_files(dir_path: &str) -> Result<Vec<String>, CustomError> {
    let mut files: Vec<String> = Vec::new();
    let entries = std::fs::read_dir(dir_path)?;
    let correct_entries = entries.map(Result::unwrap);
    for entry in correct_entries {
        if entry_is_symlink(&entry) { continue }
        if entry_is_dir(&entry) {
            let mut inner_files = collect_all_files(entry.path().to_str().expect("could not parse path from direntry"))?;
            files.append(&mut inner_files);
            continue
        }
        files.push(entry.path().to_string_lossy().to_string());
    }
    println!("{:?}", files);
    Ok(files)
}

fn is_manageable(path: &str) -> bool {
    match Image::derive_format(&path.to_string()) {
        ImageFormat::PNG => true,
        _ => false
    }
}
fn entry_is_symlink(path: &DirEntry) -> bool {
    let metadata = path.metadata().expect("error parsing metadata from direntry");
    metadata.is_symlink()
}
fn entry_is_dir(path: &DirEntry) -> bool {
    let metadata = path.metadata().expect("error parsing metadata from direntry");
    metadata.is_dir()
}

#[cfg(test)]
mod test {

}