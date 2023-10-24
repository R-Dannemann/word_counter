use std::io;
use std::fs::File;
use std::io::prelude::*;
use std::env;
use std::collections::HashMap;


fn get_text (path: &str) -> io::Result<String> {
    let mut file = File::open(path)?;
    let mut buffer = String::new();

    file.read_to_string(&mut buffer)?;

    Ok(buffer)
}

fn get_path_to_text() -> Option<String> {
    let mut args = env::args();
    args.next();
    if let Some(path_to_txt) = args.next() {
        return Some(path_to_txt);
    }
    else {
        println!("No path provided.");
        return None;
    }

}

fn text_formating(text: String) -> String {
    let text_buffer = text.to_lowercase().chars().filter(
        |text| text.is_alphabetic() || text.is_whitespace()).collect();
    return text_buffer

}

fn main()  {
    let mut content = String::new(); 

    if let Some(path_to_txt) = get_path_to_text() {
        let path_as_str: &str = &path_to_txt.as_str();

        match get_text(path_as_str) {
            Ok(data) => { content = data.clone();},
            Err(error) => eprintln!("Error reading file: {}", error),
        }
    }

    let formatet_text = text_formating(content);

    let mut hashmap_text: HashMap<&str, i32> = HashMap::new();
    let words: Vec<&str> = formatet_text.split_whitespace().collect();

    for word in words.iter() {
        if hashmap_text.contains_key(word) {
            *hashmap_text.entry(word).or_insert(1) += 1;
        }
        else {
            hashmap_text.insert(word, 1);
        }
    } 
    
    let mut sorted_text: Vec<(&&str, i32)> = hashmap_text.iter().map(|(k, &v)| (k, v)).collect();
    sorted_text.sort_by_key(|&(_, value)| std::cmp::Reverse(value));
    for &(key, value) in sorted_text.iter().take(10) {
        println!("Key: {}, Value: {}", key, value);
    }

}
// für build release "cargo build --release"