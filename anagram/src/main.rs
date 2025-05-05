use std::{collections::HashMap, fs};

fn read_txt(filepath: &str) -> String {
    let content = fs::read_to_string(filepath)
        .expect("Should have been able to read the file");
    return content;
}

fn is_subset(first: &HashMap<char, u32>, second: &HashMap<char, u32>) -> bool {
    for (key, value) in second {
        match first.get(key) {
            Some(v) if v >= value => continue,
            _ => return false,
        }
    }
    true
}

// fn split(string: &String, by: &str) -> Vec<String> {
//     string.split(by).map(|s| s.to_string()).collect()
// }

// fn trim(string: &String) -> String {
//     string.trim().to_string()
// }

// fn process_string(string: &str) -> Vec<String> {
//     string.split("  ")
//         .map(|s| s.trim().to_string())
//         .filter(|s| &s.to_string() != "")
//         .collect()
// }

// TODO
// funzione che confronta due hashmap, ritorna true se il secondo è contenuto nel primo

fn main() {
    
    let filepath = String::from("vocabulary.txt");

    let start_word = "documenting"; 
    let mut start_word_letters_count: HashMap<char, u32> = HashMap::new();

    for ch in start_word.chars() {
        *start_word_letters_count.entry(ch).or_insert(0) += 1;
    }
    
    let vocabs: Vec<String> = read_txt(&filepath)
        .split("  ")
        .map(|s| s.trim().to_string())
        .filter(|s| &s.to_string() != "")
        .collect();

    let mut filtered_vocabs: Vec<String> = Vec::new();

    for vocab in vocabs {
        let mut vocab_letters_count: HashMap<char, u32> = HashMap::new();
        for vocab_char in vocab.chars() {
            *vocab_letters_count.entry(vocab_char).or_insert(0) += 1;
        }
        if is_subset(&start_word_letters_count, &vocab_letters_count) {
            filtered_vocabs.push(vocab.to_string());
        }
    }

    // println!("{:?}", vocabs);
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashMap;

    // #[test]
    // fn test_process_string() {
    //     assert_eq!(process_string("a  b  \nc"), ["a", "b", "c"])
    // }

    // TODO cercare il modo in cui si verifica che fallisca

    #[test]
    fn subset_true() {
        let mut first: HashMap<char, u32> = HashMap::new();
        first.insert('a', 2);
        first.insert('b', 1);

        let mut second: HashMap<char, u32> = HashMap::new();
        second.insert('a', 1);
        second.insert('b', 1);
        assert_eq!(is_subset(&first, &second), true);
    }

    #[test]
    fn subset_false() {
        let mut first: HashMap<char, u32> = HashMap::new();
        first.insert('a', 2);
        first.insert('b', 1);

        let mut second: HashMap<char, u32> = HashMap::new();
        second.insert('a', 3);
        second.insert('b', 1);
        assert_eq!(is_subset(&first, &second), false);        
    }
}