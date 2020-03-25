use std::io::prelude::*;
use std::fs::File;
mod stripfile;
use stripfile::stripfile;
use counter::Counter;

/*
The main function

*/
fn main() {

    let input_path = ::std::env::args().nth(1).expect("No file found"); // Getting input file
    let input_file =  File::open(&input_path).expect(&format!("Could not open file {}", input_path));
    let mut output_file = File::create("output.txt").expect("Could not create output file");
    let contents = stripfile(input_file);
    let words = &contents.split_whitespace().collect::<Counter<_>>().most_common(); // Organizing data into a Counter collection

    for ele in words
        {
            write!(output_file, "{:?}\n" ,ele).expect("Could not write to file");
        }
    let unique_words = words.len();
    let total_words = contents.split_whitespace().count();
    write!(output_file, "Number of unique words: {}\n" ,unique_words).expect("Error");
    write!(output_file, "Number of total words: {}\n" ,total_words).expect("Error");
    println!("output.txt created");
    //  write!(p, "Percent of words needed to know to understand 100% of text: {}" ,(unique_words as f64).div(total_words as f64)).expect("Error");
}




