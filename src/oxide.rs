use std::{path::PathBuf, fs::copy, io::Error};

use io::{list_files, Transform, Transforms};
use maud::html;
use templater::{PostTemplate, Buildable, HomeTemplate};

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

    /// Post output directory
    #[clap(short, long, default_value = "posts")]
    out: String,

    #[clap(short, long, default_value = "public/")]
    root: String
}

fn main() -> Result<(), Error>{
    let args = Args::parse();

    let post_list: Vec<PathBuf> = list_files(args.posts.as_str());
    let templates_paths: Vec<PathBuf> = list_files(args.templates.as_str());
    let templates: Vec<&str> = templates_paths.iter().map(|p| {p.file_name().unwrap().to_str().unwrap()}).collect();

    let templates = PostTemplate::new_vec(post_list, templates, args.templates.as_str());
    let mut post_list_nav = String::new();

    for (_, config) in &templates {
        let post_path = format!("{}/{}.html", args.out, config.title.clone().transform(Transforms::Lowercase).transform(Transforms::NoWhitespaces));
        let markup = html! {
            li {
                time .mid-gray.tracked{
                    (config.date);
                }
                a href=(post_path) .dib.pl2.blue.dim {
                    h2 .f5.dib.blue.dim {
                        (config.title);
                    }
                }
            } 
        };
        post_list_nav.push_str(&markup.into_string());
    }

    templates.build(&format!("{}{}", args.root, args.out))?;
    let home = HomeTemplate::new(&format!("{}/home.html", args.templates), post_list_nav.to_string())?;
    home.build(&args.root)?;

    Ok(())
}