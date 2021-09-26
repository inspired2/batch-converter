mod lib;
use lib::*;

fn main() -> Result<(), &'static dyn std::error::Error>{
    let mut runtime_args = std::env::args();
    //first arg is always path-to-executable:
    let execution_path = runtime_args.next().unwrap();
    let execution_dir= trim_filename(&execution_path);
    process_all(&execution_dir).ok();
    Ok(())
}

fn trim_filename(full_path: &String) -> String {
    let path = std::path::Path::new(&full_path);
    let dir_path = path.parent().unwrap();
    dir_path.to_owned().to_str().unwrap().to_string()
}