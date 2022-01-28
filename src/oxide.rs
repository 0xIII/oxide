mod templater;
mod io;

use std::fs::File;
use std::io::Read;
use mdlib::{ToHtml, Node::*, ListType, Document, Node, Token};
use crate::io::lines;

fn main() {
    let res = lines(".\\_posts\\test.md")
        .unwrap();
    let mut s_vec: Vec<String> = Vec::new();
    for line in res {
        if let Ok(lin) = line {
            &s_vec.push(lin);
        }
    }

    Document::parse(s_vec);
}
