use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
extern crate ureq;
use ureq::{json, serde_json::Value};
fn main() {
    // Read the file path from command line arguments
    let input_path = std::env::args().nth(1).expect("No file found");

    // Open the input file
    let input_file =
        File::open(&input_path).unwrap_or_else(|_| panic!("Could not open file {}", input_path));
    let reader = BufReader::new(input_file);

    // Create the output file
    let mut output_file = File::create("output.txt").expect("Could not create output file");

    // Read all lines from the input file
    let mut non_translated_string = String::new();
    for line in reader.lines() {
        match line {
            Ok(read_line) => {
                non_translated_string += &(read_line + "\n");
            }
            Err(e) => {
                eprintln!("Could not read a line from the file: {}", e);
            }
        }
    }

    // Translate the non-translated string
    if let Ok(translated_string) = translate(non_translated_string.clone()) {
        let striped_string = stripstring(non_translated_string.clone());

        // Count word frequencies using HashMap
        let mut word_count: HashMap<&str, usize> = HashMap::new();
        for word in striped_string.split_whitespace() {
            *word_count.entry(word).or_insert(0) += 1;
        }

        // Sort the HashMap by frequency (value), in descending order
        let mut word_count_vec: Vec<(&str, usize)> =
            word_count.iter().map(|(k, v)| (*k, *v)).collect();

        word_count_vec.sort_by(|a, b| b.1.cmp(&a.1));

        // Translate the organized words
        let mut translated_organized_words = String::new();
        for (word, _) in &word_count_vec {
            translated_organized_words += &(word.to_string() + "\n");
        }

        if let Ok(translated_organized_string) = translate(translated_organized_words) {
            translated_organized_words = translated_organized_string;
            let vector_translated_organized_words: Vec<_> =
                translated_organized_words.split('\n').collect();

            for ((word, count), translated_word) in word_count_vec
                .iter()
                .zip(vector_translated_organized_words.iter())
            {
                write!(output_file, "({:?}, ", word).expect("Could not write to file");
                write!(output_file, "{:?})", count).expect("Could not write to file");
                writeln!(output_file, " {:?}", translated_word).expect("Could not write to file");
            }

            let unique_words = word_count.len();
            let total_words = striped_string.split_whitespace().count();

            writeln!(output_file, "Number of unique words: {}", unique_words)
                .expect("Could not write to file");
            writeln!(output_file, "Number of total words: {}", total_words)
                .expect("Could not write to file");

            let translated_vec: Vec<_> = translated_string.split('\n').collect();
            for (phrase, translated_phrase) in
                non_translated_string.split('\n').zip(translated_vec.iter())
            {
                if !phrase.is_empty() {
                    writeln!(output_file, "{}", translated_phrase)
                        .expect("Could not write to file");
                    writeln!(output_file).expect("Could not write to file");
                }
            }
        } else {
            println!("Could not translate organized words");
        }
    } else {
        println!("Could not translate the text");
    }

    println!("output.txt created");
}

fn stripstring(mut input_str: String) -> String {
    input_str.make_ascii_lowercase();
    let allowed_chars = "abcdefghijklmnopqrstuvwxyzáéíóúüñçâêîôûäëïöüàèìòùãõåæœ ";
    let contents: String = input_str
        .chars()
        .filter(|&c| allowed_chars.contains(c))
        .collect();
    contents
}
fn translate(input_phrase: String) -> Result<String, String> {
    let resp = ureq::post("https://translation.googleapis.com/language/translate/v2?key=AIzaSyBhB3zceTYVekQ93GqzdVfQLYNhaRoon00")
        .set("Content-Type", "application/json; charset=utf-8")
        .send_json(json!({
            "q": input_phrase,
            "target": "en",
            "format": "text"
        }));

    match resp {
        Ok(successful_resp) => {
            let translation_result: Result<Value, _> = successful_resp.into_json();
            match translation_result {
                Ok(json_value) => {
                    let translation: &Value =
                        &json_value["data"]["translations"][0]["translatedText"];
                    match translation.as_str() {
                        Some(translated_str) => Ok(String::from(translated_str)),
                        None => Err("Translation could not be converted to string".to_string()),
                    }
                }
                Err(_) => Err("Failed to parse response into JSON".to_string()),
            }
        }
        Err(_) => Err("Failed to make a request".to_string()),
    }
}
