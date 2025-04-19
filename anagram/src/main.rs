use std::fs;

// fn read_txt(filepath: &str) -> String {
//     let content = fs::read_to_string(filepath)
//         .expect("Should have been able to read the file");
//     return content;
// }

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

fn main() {
    // let filepath = String::from("vocabulary.txt");
    // let content = read_txt(&filepath);
    // // let mut vocab = split(&content, "  ");
    // let vocab: Vec<String> = content.split("  ")
    //     .map(|s| s.trim().to_string())
    //     .filter(|s| &s.to_string() != "")
    //     .collect();
    // // vocab = vocab.iter()
    // //     .map(|s| trim(&s))
    // //     .filter(|s| &s.to_string() != "")
    // //     .collect();

    // println!("{:?}", vocab);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process_string() {
        assert_eq!(process_string("a  b  \nc"), ["a", "b", "c"])
    }

    // TODO cercare il modo in cui si verifica che fallisca
}

fn process_string(string: &str) -> Vec<String> {
    string.split("  ")
        .map(|s| s.trim().to_string())
        .filter(|s| &s.to_string() != "")
        .collect()
}