use std::io;
use std::io::Error;

type Line = String;
type Tokenstream = Vec<String>;

pub enum TFlags {
    CODE,
    LIST,
    NONE
}

// ToHtml Trait for all elements that will be "rendered" to html
pub trait ToHtml {
    fn to_html(self) -> Result<String, Error>;
}

pub trait Token<T> {
    fn tokenize(&self, flag: TFlags) -> (Box<T>, TFlags);
}

// Top-level Markdown document data
pub struct Document {
    top: Vec<Node>
}

impl Document {
    pub fn parse(markdown: Vec<String>) {
        let mut doc = Document {
            top: Vec::new()
        };

        for line in markdown {
            if line.len() != 0 {
                doc.top.push(line.tokenize(TFlags::NONE).0.parse());
            }
        }
    }
}

impl ToHtml for Document {
    fn to_html(self) -> Result<String, Error> {
        todo!()
    }
}

#[derive(Debug)]
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

// enum of all supported Markdown Elements
#[derive(Debug)]
pub enum Node {
    Bold(Box<Node>), //**
    Italics(Box<Node>), //*
    Heading{ inner: Box<Node>, size: u8 }, // # ## ###
    Text(String),
    Code(Box<Node>), // ``
    BlockQuote(Box<Node>), // >
    List{ items: Vec<Box<Node>>, listtype: ListType}, // -
    ListItem(Box<Node>), // -
    Link{ text: String, link: String }, // []()
    Image{ text: String, link: String }, // ![]()
    HR, // ---
}

impl Node {
    pub fn parse(&self) -> Node {
        println!("{:?}", self);

        let mut prev: &Node = self;

    }
}

impl Token<Node> for Line {
    fn tokenize(&self, flag: TFlags) -> (Box<Node>, TFlags) {
        let words: Vec<&str> = self.split_whitespace().collect();
        let mut stripped: String = String::from("");
        for word in &words[1..words.len()] { stripped.push_str(word); }

        match words[0] {
            "#" => {
                (Box::new(Node::Heading { inner: Box::new(Node::Text(stripped)), size: 1 }), flag)
            },
            "##" => {
                (Box::new(Node::Heading { inner: Box::new(Node::Text(stripped)), size: 2 }), flag)
            },
            "###" => {
                (Box::new(Node::Heading { inner: Box::new(Node::Text(stripped)), size: 3 }), flag)
            },
            ">" => {
                (Box::new(Node::BlockQuote(Box::new(Node::Text(stripped)))), flag)
            },
            "---" => {
                (Box::new(Node::HR), flag)
            },
            "```" => {
                (Box::new(Node::Code(Box::new(Node::Text(stripped)))), TFlags::CODE)
            }
            _ => {
                (Box::new(Node::Text(self.to_string())), flag)
            }
        }
    }
}

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
                 Ok(format!("<pre><code>{}</code></pre>", inner.to_html()?))
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