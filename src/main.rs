fn main() {
    let path: Option<String> = std::env::args().nth(1);
    match path {
        Some(x) => process(x),
        None => println!("missing path as argument")
    }
}

fn process(path_string: String)  {
    match get_content(&path_string) {
        Ok(content) => print_stats(content),
        Err(error) => println!("{error}")
    }
}

fn get_content(path_string: &str) -> Result<String, pdf_extract::OutputError> {

    let path: &std::path::Path = std::path::Path::new(&path_string);
    pdf_extract::extract_text(path)
}

fn print_stats(content: String) {
    for a in content.split_whitespace() {
        println!("{a}");
    }
}

fn trim_nonalphabetical_front_and_back(source: &str) -> &str {
    source
}

mod tests {
    use super::*;

    #[test]
    fn trim_does_not_modify_alphabetical() {
        assert_eq!(trim_nonalphabetical_front_and_back("word"), "word");
    }

    #[test]
    fn trim_removes_front() {
        assert_eq!(trim_nonalphabetical_front_and_back("hello"), "12hello");
        assert_eq!(trim_nonalphabetical_front_and_back("foo"), "-foo");
    }
 
    #[test]
    fn trim_removes_back() {
        assert_eq!(trim_nonalphabetical_front_and_back("hello"), "hello34");
        assert_eq!(trim_nonalphabetical_front_and_back("foo"), "foo+");
    }

    #[test]
    fn preserved_middle() {
        assert_eq!(trim_nonalphabetical_front_and_back("hello-world"), "13hello-world)");
    }
}
