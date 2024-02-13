use std::{env, fs};

fn load_wordlist(filename: &str) -> Vec<String> {
    let contents = fs::read_to_string(filename).expect("should have been able to read the file");

    let mut vec = vec![];
    for line in contents.split('\n') {
        vec.push(line.to_string());
    }
    vec
}

fn filter_string(string: String) -> String {
    string
        .chars()
        .filter_map(|char| match char {
            'ä' => Some("ae".to_string()),
            'ö' => Some("oe".to_string()),
            'ü' => Some("ue".to_string()),
            'ß' => Some("ss".to_string()),
            ch if ch.is_ascii_lowercase() => Some(format!("{ch}")),
            _ => None,
        })
        .collect()
}

fn sort_string(string: String) -> String {
    let mut chars: Vec<char> = string.chars().collect();
    chars.sort();
    String::from_iter(chars)
}

fn comparable_string(string: String) -> String {
    sort_string(filter_string(string.to_lowercase()))
}

fn find_anagram_solution(anagram: String, wordlist: Vec<String>) -> Option<String> {
    let sorted_anagram = comparable_string(anagram);

    for word in wordlist {
        let sorted_word = comparable_string(word.to_string());
        if sorted_word == sorted_anagram {
            return Some(word);
        }
    }

    None
}

fn find_ddf_case_by_anagram(anagram: String) -> Option<String> {
    let wordlist_paths = vec![
        "resources/books_list.txt",
        "resources/episodes_list.txt",
        "resources/graphic_novels_list.txt",
        "resources/planetarium_list.txt",
    ];

    for path in wordlist_paths {
        let wordlist = load_wordlist(path);
        let found = find_anagram_solution(anagram.clone(), wordlist);
        if let Some(solution) = found {
            return Some(solution);
        }
    }
    None
}

fn main() {
    let args: Vec<String> = env::args().collect();
    let anagram = if args.len() > 1 { args[1].as_str() } else { "" };
    let found = find_ddf_case_by_anagram(anagram.to_string());
    println!("Anagram: \"{}\"", anagram);
    if let Some(solution) = found {
        println!("Solution: {}", solution);
    } else {
        println!("No solution found :c");
    }
}
