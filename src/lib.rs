mod tests;

use std::io;
use std::io::Error;
use std::path::Path;
use std::thread::current;
use crate::Node::Text;

// ToHtml Trait for all elements that will be "rendered" to html
pub trait ToHtml {
    fn to_html(self) -> Result<String, Error>;
}

pub trait Parser {
    fn parse(self) -> Box<Node>;
}

// Markdown document data
pub struct Document {
    path: String,
    markdown: String,
}

impl Document {
    pub fn parse(mut markdown: String) {
    }
}

impl Parser for String {
    fn parse(self) -> Box<Node> {
        // clean up the string
        let cleaned: String = self.replace("\r\n\r", "");
        let mut markdown: Vec<&str> = cleaned.split('\n').collect();

        for line in markdown {
            let stream: Vec<&str> = line.split(' ').collect();
            for word in stream {
                println!("{}", word);

                // check if word is a line token
                match word {
                    // Heading 1
                    "#" => { return Box::new(Node::Heading { inner: line[2..].to_string().parse(), size: 1 }); },
                    // Heading 2
                    "##" => { return Box::new(Node::Heading { inner: line[3..].to_string().parse(), size: 2 }); },
                    // Heading 3
                    "###" => { return Box::new(Node::Heading { inner: line[4..].to_string().parse(), size: 3 }); },
                    // Blockquote
                    ">" => { return Box::new(Node::BlockQuote(
                        line[2..].to_string().parse()
                    )); },
                    "-" => {},
                    "```" => {},
                    "---" => { return Box::new(Node::HR); },
                    _ => {
                        // check every word in line
                        match &word[..1] {
                            "*" => {
                                return if &word[..2] == "**" {
                                    // ** bold
                                    Box::new(Node::Bold(word[2..word.len()-2].to_string().parse()))
                                } else {
                                    // italics
                                    Box::new(Node::Italics(word[1..word.len()-1].to_string().parse()))
                                }
                            },
                            "!" => {
                                // image ![alt text](link)
                                if let (Some(a), Some(b)) = (word.find("["), word.find("]")) {
                                    if let (Some(c), Some(d)) = (word.find("("), word.find(")")) {
                                        let text: &str = &word[a + 1..b];
                                        let link: &str = &word[c + 1..d];
                                        return Box::new(Node::Image { text: text.to_string(), link: link.to_string() });
                                    }
                                }
                            },
                            "[" => {
                                // hyperlink [text](link)
                                if let (Some(a), Some(b)) = (word.find("["), word.find("]")) {
                                    if let (Some(c), Some(d)) = (word.find("("), word.find(")")) {
                                        let text: &str = &word[a + 1..b];
                                        let link: &str = &word[c + 1..d];
                                        return Box::new(Node::Link { text: text.to_string(), link: link.to_string() });
                                    }
                                }
                            },
                            _ => {
                                // Pure text
                                return Box::new(Node::Text(word.to_string()));
                            }
                        }
                    }
                }
            }
        }
        Box::new(Node::Text("".to_string()))
    }
}

impl ToHtml for Document {
    fn to_html(self) -> Result<String, Error> {
        todo!()
    }
}

#[derive(Debug, PartialOrd, PartialEq)]
pub enum ListType {
    Ordered,
    Unordered
}

impl ToHtml for ListType {
    fn to_html(self) -> Result<String, Error> {
        match self {
            ListType::Ordered => {Ok(String::from("ol"))}
            ListType::Unordered => {Ok(String::from("ul"))}
        }
    }
}

/*
enum of all supported Markdown Elements
*/
#[derive(Debug, PartialOrd, PartialEq)]
pub enum Node {
    Bold(Box<Node>), //**
    Italics(Box<Node>), //*
    Heading{ inner: Box<Node>, size: u8 }, // # ## ###
    Text(String),
    Code(String), // ```
    BlockQuote(Box<Node>), // >
    List{ items: Vec<Box<Node>>, listtype: ListType}, // -
    ListItem(Box<Node>), // -
    Link{ text: String, link: String }, // []()
    Image{ text: String, link: String }, // ![]()
    HR, // ---
}

impl Node {
    pub fn parse(&self) {

    }
}

/*
Convert a Node object to its corresponding html
*/
impl ToHtml for Node {
    fn to_html(self) -> Result<String, Error> {
         match self {
             Node::Bold(inner) => {
                 Ok(format!("<b>{}</b>", inner.to_html()?))
             },
             Node::Italics(inner) => {
                 Ok(format!("<i>{}</i>", inner.to_html()?))
             },
             Node::Heading { inner, size } => {
                Ok(format!("<h{sz}>{cn}</h{sz}>", cn = inner.to_html()?, sz = size))
             },
             Node::Text(text) => {
                Ok(text.to_string())
             },
             Node::Code(inner) => {
                 Ok(format!("<pre><code>{}</code></pre>", inner))
             },
             Node::BlockQuote(inner) => {
                 Ok(format!("<blockquote>{}</blockquote>", inner.to_html()?))
             },
             Node::List {items, listtype} => {
                 let mut listcn: String = String::from("");
                 for item in items {
                     listcn.push_str(&item.to_html().unwrap_or_else(|_| {String::from("")}));
                 }
                 Ok(format!("<{ltype}>{list}</{ltype}>", ltype = listtype.to_html()?, list = listcn))
             },
             Node::ListItem(inner) => {
                 Ok(format!("<li>{}</li>", inner.to_html()?))
             },
             Node::Link {text, link} => {
                Ok(format!("<a href='{}'>{}</a>", link, text))
             },
             Node::Image {text, link} => {
                Ok(format!("<img src='{}' alt='{}'>", link, text))
             },
             Node::HR => {
                 Ok(String::from("<hr>"))
             },
             _ => {
                 Err(io::Error::new(io::ErrorKind::Unsupported, "Unsupported formatting"))
             }
         }
    }
}