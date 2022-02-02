mod templater;
mod io;

use std::fs::File;
use std::io::Read;
use std::str::{from_utf8, from_utf8_unchecked};
use mdlib::{ToHtml, Node::*, ListType, Document, Node, Parser};
use crate::io::{read_string, read_vec};

fn main() {
    let res = read_string(".\\_posts\\test.md")
        .unwrap();
    let test = "![text](link)".to_string();
    test.parse();
    //Document::parse(res);
}
