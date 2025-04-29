use std::{collections::HashMap, fs};

fn read_txt(filepath: &str) -> String {
    let content = fs::read_to_string(filepath)
        .expect("Should have been able to read the file");
    return content;
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
    let content = read_txt(&filepath);

    let start_word = "documenting"; 
    let mut start_word_letters_count: HashMap<char, u32> = HashMap::new();

    for ch in start_word.chars() {
        *start_word_letters_count.entry(ch).or_insert(0) += 1;
    }
    
    let vocabs: Vec<String> = content.split("  ")
        .map(|s| s.trim().to_string())
        .filter(|s| &s.to_string() != "")
        .collect();

    for vocab in &vocabs {
        let mut vocab_letters_count: HashMap<char, u32> = HashMap::new();
        for vocab_char in vocab.chars() {
            *vocab_letters_count.entry(vocab_char).or_insert(0) += 1;
        }

    }

    println!("{:?}", vocabs);
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_process_string() {
    //     assert_eq!(process_string("a  b  \nc"), ["a", "b", "c"])
    // }

    // TODO cercare il modo in cui si verifica che fallisca
}