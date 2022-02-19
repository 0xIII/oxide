use std::{path::PathBuf, fs::copy};

use io::list_files;
use templater::{Template, Buildable};

use clap::Parser;

mod templater;
mod io;

/// Oxide Static Site Generator
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Directory containing the markdown posts
    #[clap(short, long, default_value = "_posts")]
    posts: String,

    /// Template Directory
    #[clap(short, long, default_value = "_templates")]
    templates: String,

    /// Output directory
    #[clap(short, long, default_value = "public/posts")]
    out: String,
}

fn main() {
    let args = Args::parse();

    let post_list: Vec<PathBuf> = list_files(args.posts.as_str());
    let mut templates_paths: Vec<PathBuf> = list_files(args.templates.as_str());
    let mut templates: Vec<&str> = templates_paths.iter().map(|p| {p.file_name().unwrap().to_str().unwrap()}).collect();

    let templates = Template::new_vec(post_list, templates, args.templates.as_str());
    templates.build(args.out.as_str());
}