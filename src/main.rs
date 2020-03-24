use std::io::prelude::*;
use std::fs::File;
extern crate regex;
use regex::Regex;
use counter::Counter;
/*
This function will remove all unnecessary chars from the input file i.e  punctuation, digits and some spanish chars
It will also turn the input text all lowercase

*/
fn stripfile(mut input_file: File) ->  std::string::String
{   let mut contents = String::new();
    let strip_misc = Regex::new(r"[[:punct:][:digit:][¿¡]]").unwrap();
    input_file.read_to_string(&mut contents).expect("no");
    contents.make_ascii_lowercase();
    let contents = strip_misc.replace_all(&contents,"");
    contents.into_owned()

}

fn main() {

    let input_path = ::std::env::args().nth(1).unwrap(); // Getting input file

    let input_file =  File::open(&input_path).unwrap();

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




