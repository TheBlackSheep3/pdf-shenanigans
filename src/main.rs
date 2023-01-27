use std::collections::HashMap;

fn main() {
    let path: Option<String> = std::env::args().nth(1);
    let count: usize = match std::env::args()
        .nth(2)
        .and_then(|x| x.parse::<usize>().ok())
    {
        Some(val) => val,
        None => 10,
    };
    match path {
        Some(x) => process(x, count),
        None => println!("missing path as argument"),
    }
}

fn process(path_string: String, count: usize) {
    match get_content(&path_string) {
        Ok(content) => print_stats(content, count),
        Err(error) => println!("{error}"),
    }
}

fn get_content(path_string: &str) -> Result<String, pdf_extract::OutputError> {
    let path: &std::path::Path = std::path::Path::new(&path_string);
    pdf_extract::extract_text(path)
}

fn print_stats(content: String, count: usize) {
    let mut map = HashMap::<&str, u32>::new();
    for word in content
        .split_whitespace()
        .map(|x| trim_nonalphabetic_front_and_back(x))
        .filter(|x| !x.is_empty())
    {
        let count = map.entry(word).or_insert(0u32);
        *count += 1;
    }
    let total_count: Option<u32> = map.iter().try_fold(0u32, |acc, (_, &x)| acc.checked_add(x));
    let mut vec: Vec<(&str, u32)> = map.into_iter().collect();
    vec.sort_by(|(_, l), (_, r)| r.cmp(l));
    let top: usize = if vec.len() < count { vec.len() } else { count };
    match total_count {
        Some(sum) => println!("total word count: {}", sum),
        None => println!("An error occured while counting words"),
    }
    for t in &vec[..top] {
        match total_count {
            Some(sum) => println!(
                "{0}: {1} times ({2:.2} %)",
                t.0,
                t.1,
                (t.1 as f32) / (sum as f32) * 100f32
            ),
            None => println!("{0}: {1} times", t.0, t.1),
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
