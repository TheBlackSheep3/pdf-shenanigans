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
