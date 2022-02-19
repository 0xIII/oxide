use std::{path::PathBuf, ffi::{OsStr, OsString}};

use io::list_files;
use templater::{Conf, Template, Buildable};

mod templater;
mod io;

const POST_DIR: &str = "_posts";
const TEMPLATE_DIR: &str = "public/_templates";

fn main() {
    let post_list: Vec<PathBuf> = list_files(POST_DIR);
    let mut templates_paths: Vec<PathBuf> = list_files(TEMPLATE_DIR);
    let mut templates: Vec<&str> = templates_paths.iter().map(|p| {p.file_name().unwrap().to_str().unwrap()}).collect();

    let templates = Template::new_vec(post_list, templates, TEMPLATE_DIR);
    templates.build();
}