use std::fs::{DirEntry, File, read_dir, ReadDir};
use std::io::{prelude::*, Result, Lines, BufReader};
use std::path::{Path, PathBuf};

pub fn list_files(dir: &str) -> Vec<PathBuf> {
    let mut pathes: Vec<PathBuf> = Vec::new();
    let files = read_dir(dir)
        .unwrap();
    for path in files {
        pathes.push(path.unwrap().path());
    }
    pathes
}

pub fn read_string<P: AsRef<Path>>(filename: P) -> Result<String> {
    let mut buffer = String::new();
    let mut handle = File::open(&filename).unwrap_or_else(|_| {
        File::create(&filename).expect("Unable to create file")
    });
    handle.read_to_string(&mut buffer)?;
    Ok(buffer)
}

pub fn read_vec<P: AsRef<Path>>(filename: P) -> Result<Vec<u8>> {
    let mut buffer = Vec::new();
    let mut handle = File::open(&filename).unwrap_or_else(|_| {
        File::create(&filename).expect("Unable to create file")
    });
    handle.read_to_end(&mut buffer)?;
    Ok(buffer)
}

pub fn lines<P: AsRef<Path>>(filename: P) -> Result<Lines<BufReader<File>>> {
    let handle = File::open(&filename).unwrap_or_else(|_| {
        File::create(&filename).expect("Unable to create file")
    });
    Ok(BufReader::new(handle).lines())
}