#![feature(is_symlink)]
extern crate rayon;
mod lib;
use lib::*;
use std::io::stdin;

type CustomError = Box<dyn std::error::Error>;


fn main() -> Result<(), CustomError>{
    let mut runtime_args = std::env::args();
    let args_length = runtime_args.len();
    //first arg is always path-to-executable:
    let execution_path = runtime_args.next().unwrap();

    let execution_dir= trim_filename(&execution_path);
    //if argument provided it is considered to be absolute path to the target directory
    if args_length > 1 {process_all(runtime_args.last().unwrap())?;}
    else {
        process_all(execution_dir)?;}
    println!("обработка завершена, нажмите любую клавишу, чтобы закрыть окно");
    let buf = &mut String::new();
    loop {
        let _input = stdin().read_line(buf);
        if buf.len() > 0 {break}
    }
    Ok(())
}

fn trim_filename(full_path: &String) -> String {
    let path = std::path::Path::new(&full_path);
    let dir_path = path.parent().unwrap();
    dir_path.to_owned().to_str().unwrap().to_string()
}