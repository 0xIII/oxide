use std::{io::Error, fs::write, path::PathBuf};
use lazy_static::lazy_static;
use regex::Regex;
use serde::{Deserialize, Serialize};

use crate::io::{Transform, Transforms, read_string};

type DOM = String;

pub trait Buildable {
    fn build(self, out: &str) -> Result<(), Error>;
}

#[derive(Debug, Default, Serialize, Deserialize)]
pub struct Conf {
    pub title: String,
    pub date: String,
    pub template: String,
}

#[derive(Default, Debug)]
pub struct HomeTemplate {
    pub template: String,
    pub markup: String,
}

impl HomeTemplate {
    pub fn new(template_path: &str, markup: String) -> std::io::Result<HomeTemplate> {
        Ok(
            HomeTemplate {
                template: read_string(template_path)?,
                markup
            }
        )
    }
}

impl Buildable for HomeTemplate {
    fn build(self, out: &str) -> Result<(), Error> {
        let filepath = format!("{}/home.html", out);
        let dom: DOM = self.template
            .replace("{{list}}", &self.markup);
        write(filepath, dom)?;

        println!("[+] Built home.html");

        Ok(())
    }
}

#[derive(Default, Debug)]
pub struct PostTemplate {
    pub template: String,
    pub markdown: String,
}

impl Buildable for (PostTemplate, Conf) {
    fn build(self, out: &str) -> Result<(), Error> {
        let filepath = format!("{}/{}.html", out, self.1.title.clone().transform(Transforms::Lowercase).transform(Transforms::NoWhitespaces));
        let html: DOM = markdown::to_html(self.0.markdown.as_str());
        let dom: DOM = self.0.template
            .replace("{{title}}", self.1.title.as_str())
            .replace("{{date}}", self.1.date.as_str())
            .replace("{{content}}", html.as_str());
        write(filepath, dom)?;

        println!("[+] Built post '{}' with template '{}'", &self.1.title, &self.1.template);

        Ok(())
    }
}

impl Buildable for Vec<(PostTemplate, Conf)> {
    fn build(self, out: &str) -> Result<(), Error> {
        for template in self {
            template.build(out)?;
        }
        Ok(())
    }
}

impl PostTemplate {
    pub fn new(template: DOM, markdown: String)  -> Self {
        PostTemplate {
            template,
            markdown,
        }
    }

    pub fn new_vec(posts: Vec<PathBuf>, templates: Vec<&str>, template_dir: &str) -> Vec<(Self, Conf)> {
        let mut templater: Vec<(Self, Conf)> = Vec::new();

        lazy_static! {
            static ref CONF_REGEX: Regex = Regex::new(r"---([\s\S]*?)----([\s\S]*?)----").unwrap();
        }

        for post in posts {
            let config: Conf;
            let content = read_string(&post).unwrap();
            
            if let Some(caps) = CONF_REGEX.captures(&content) {
                config = serde_yaml::from_str(&format!("---{}", caps.get(1).unwrap().as_str()))
                    .unwrap();

                if templates.contains(&&config.template[..]) {
                    let html: DOM = read_string(format!("{}/{}", template_dir, config.template)).unwrap();

                    let markdown: String = caps.get(2).unwrap().as_str().to_string();

                    templater.push((PostTemplate {
                        template: html,
                        markdown,
                    }, serde_yaml::from_str(&format!("---{}", caps.get(1).unwrap().as_str()))
                    .unwrap()));

                    println!("[+] Added post '{}' with template '{}'", post.to_str().expect("Invalid path format"), &config.template[..]);
                } else {
                    eprintln!("[!] Unable to find template '{}' in '{}'", &config.template[..], post.to_str().expect("Invalid path format"));
                }
            }
        }

        templater
    }
}