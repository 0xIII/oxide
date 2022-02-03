#[cfg(test)]
mod tests {
    use crate::{Node, Parser};

    #[test]
    fn test() {
        let text =  String::from("test text");
        assert_eq!(text.parse(), Box::new(Node::HR));
    }

    #[test]
    fn image() {
        let image = String::from("![text](link)");
        assert_eq!(Box::new(Node::Image { text: "text".to_string(), link: "link".to_string() }), image.parse());
    }

    #[test]
    fn hyperlink() {
        let link = String::from("[text](link)");
        assert_eq!(Box::new(Node::Link { text: "text".to_string(), link: "link".to_string() }), link.parse());
    }

    #[test]
    fn multiline_code() {
        let code = "```\r\nThis is a line\r\nThis is another line\r\n```".to_string();
    }

    #[test]
    fn heading() {
        let heading = "# ## ### test".to_string();
        assert_eq!(heading.parse(), Box::new(Node::Heading { inner: Box::new(Node::Heading { inner: Box::new(Node::Heading { inner: Box::new(Node::Text("test".to_string())), size: 3 }), size: 2 }), size: 1 }));
    }

    #[test]
    fn bold() {
        let bold = "**bold**".to_string();
        assert_eq!(bold.parse(), Box::new(Node::Bold(Box::new(Node::Text("bold".to_string())))));
    }

    #[test]
    fn italic() {
        let italic = "*italic*".to_string();
        assert_eq!(italic.parse(), Box::new(Node::Italics(Box::new(Node::Text("italic".to_string())))));
    }

    #[test]
    fn blockquote() {
        let quote = "> test".to_string();
        assert_eq!(quote.parse(), Box::new(Node::BlockQuote(Box::new(Node::Text("test".to_string())))));
    }

    #[test]
    fn hr() {
        let rule = "---".to_string();
        assert_eq!(rule.parse(), Box::new(Node::HR));
    }
}