fn main() {
    let path: Option<String> = std::env::args().nth(1);
    match path {
        Some(x) => process(x),
        None => println!("missing path as argument"),
    }
}

fn process(path_string: String) {
    match get_content(&path_string) {
        Ok(content) => print_stats(content),
        Err(error) => println!("{error}"),
    }
}

fn get_content(path_string: &str) -> Result<String, pdf_extract::OutputError> {
    let path: &std::path::Path = std::path::Path::new(&path_string);
    pdf_extract::extract_text(path)
}

fn print_stats(content: String) {
    for a in content
        .split_whitespace()
        .map(|x| trim_nonalphabetic_front_and_back(x))
    {
        if !a.is_empty() {
            println!("{a}");
        }
    }
}

fn trim_nonalphabetic_front_and_back(source: &str) -> &str {
    source
        .trim_start_matches(|x: char| !x.is_alphabetic())
        .trim_end_matches(|x: char| !x.is_alphabetic())
}

#[cfg(test)]
mod tests {
    mod trim {
        use crate::trim_nonalphabetic_front_and_back;

        #[test]
        fn does_not_modify_alphabetical() {
            assert_eq!(trim_nonalphabetic_front_and_back("word"), "word");
        }

        #[test]
        fn removes_front() {
            assert_eq!(trim_nonalphabetic_front_and_back("12hello"), "hello");
            assert_eq!(trim_nonalphabetic_front_and_back("-foo"), "foo");
        }

        #[test]
        fn removes_back() {
            assert_eq!(trim_nonalphabetic_front_and_back("hello34"), "hello");
            assert_eq!(trim_nonalphabetic_front_and_back("foo+"), "foo");
        }

        #[test]
        fn preserves_middle() {
            assert_eq!(
                trim_nonalphabetic_front_and_back("13hello-world)"),
                "hello-world"
            );
        }

        #[test]
        fn empty_for_only_nonalphabetic() {
            assert_eq!(trim_nonalphabetic_front_and_back("122.3"), "");
            assert_eq!(trim_nonalphabetic_front_and_back("3-4+9"), "");
        }

        #[test]
        fn do_not_panic() {
            assert_eq!(trim_nonalphabetic_front_and_back("hiermit:○"), "hiermit");
        }
    }
}
