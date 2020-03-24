use std::io::prelude::*;
use std::fs::File;
extern crate regex;
use regex::Regex;
use counter::Counter;

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

    let v =  File::open(&input_path).unwrap();

    let mut p = File::create("output.txt").expect("Error");
    let contents = stripfile(v);
    let words = &contents.split_whitespace().collect::<Counter<_>>().most_common();

    for ele in words
        {
            write!(p, "{:?}\n" ,ele).expect("Error");
        }
    let unique_words = words.len();
    let total_words = contents.split_whitespace().count();
    write!(p, "Number of unique words: {}\n" ,unique_words).expect("Error");
    write!(p, "Number of total words: {}\n" ,total_words).expect("Error");
    println!("output.txt created");
    //  write!(p, "Percent of words needed to know to understand 100% of text: {}" ,(unique_words as f64).div(total_words as f64)).expect("Error");
}




