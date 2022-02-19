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
pub struct Template {
    pub template: String,
    pub markdown: String,
}

impl Buildable for (Template, Conf) {
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

impl Buildable for Vec<(Template, Conf)> {
    fn build(self, out: &str) -> Result<(), Error> {
        for template in self {
            template.build(out)?;
        }
        Ok(())
    }
}

impl Template {
    pub fn new(template: DOM, markdown: String)  -> Self {
        Template {
            template,
            markdown,
        }
    }

    pub fn new_vec(posts: Vec<PathBuf>, templates: Vec<&str>, template_dir: &str) -> Vec<(Self, Conf)> {
        let mut templater: Vec<(Self, Conf)> = Vec::new();

        lazy_static! {
            static ref conf_regex: Regex = Regex::new(r"---([\s\S]*?)----([\s\S]*?)----").unwrap();
        }

        for post in posts {
            let mut config: Conf;
            let content = read_string(&post).unwrap();
            
            if let Some(caps) = conf_regex.captures(&content) {
                config = serde_yaml::from_str(&format!("---{}", caps.get(1).unwrap().as_str()))
                    .unwrap();

                if templates.contains(&&config.template[..]) {
                    let html: DOM = read_string(format!("{}/{}", template_dir, config.template)).unwrap();

                    let markdown: String = caps.get(2).unwrap().as_str().to_string();

                    templater.push((Template {
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