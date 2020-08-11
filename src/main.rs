use std::fs::File;
use std::io::prelude::*;
mod stripstring;
mod translate;
//use std::collections::HashMap;
use counter::Counter;
use std::io::BufReader;
use stripstring::stripstring;
use translate::translate;
/*
The main function

*/
fn main() {
    let input_path = ::std::env::args().nth(1).expect("No file found"); // Getting input file
    let input_file =
        File::open(&input_path).unwrap_or_else(|_| panic!("Could not open file {}", input_path));
    let mut output_file = File::create("output.txt").expect("Could not create output file");
    let reader = BufReader::new(input_file);
    let mut non_translated_string = String::new();
    for line in reader.lines() {
        non_translated_string += &(line.unwrap() + "\n");
    }
    let translated_string = translate(non_translated_string.clone());
    let striped_string = stripstring(non_translated_string.clone());
    let orginized_words = &striped_string
        .split_whitespace()
        .collect::<Counter<_>>()
        .most_common(); // Organizing data into a Counter collection
    let mut translated_orginized_words = String::new();
    for i in orginized_words {
        translated_orginized_words += &(i.0.to_owned() + "\n");
    }

    translated_orginized_words = translate(translated_orginized_words);
    let vector_translated_orginized_words: Vec<_> =
        translated_orginized_words.split('\n').collect();
    for (ele, ele2) in orginized_words
        .iter()
        .zip(vector_translated_orginized_words.iter())
    {
        write!(output_file, "{:?} ", ele).expect("Could not write to file");
        writeln!(output_file, "{:?}", ele2).expect("Could not write to file");
    }
    let unique_words = orginized_words.len();
    let total_words = striped_string.split_whitespace().count();
    writeln!(output_file, "Number of unique words: {}", unique_words).expect("Error");
    writeln!(output_file, "Number of total words: {}", total_words).expect("Error");
    let non_translated_vec: Vec<_> = non_translated_string.split('\n').collect();
    let translated_vec: Vec<_> = translated_string.split('\n').collect();

    for (phrase, translated_phrase) in non_translated_vec.iter().zip(translated_vec.iter()) {
        if *phrase != "" {
            // write!(output_file, "{} " ,phrase).expect("Error");
            writeln!(output_file, "{}", translated_phrase).expect("Error");
            writeln!(output_file).expect("Error");
        }
    }

    println!("output.txt created");
}
