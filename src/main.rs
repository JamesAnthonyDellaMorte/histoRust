use std::io::prelude::*;
use std::fs::File;
mod stripstring;
mod translate;
//use std::collections::HashMap;
use translate::translate;
use stripstring::stripstring;
use counter::Counter;
use std::io::BufReader;
/*
The main function

*/
fn main() {

    let input_path = ::std::env::args().nth(1).expect("No file found"); // Getting input file
    let input_file =  File::open(&input_path).expect(&format!("Could not open file {}", input_path));
    let mut output_file = File::create("output.txt").expect("Could not create output file");
    let reader = BufReader::new(input_file);
    let mut non_translated_string = String::new();
    for line in reader.lines() {
         non_translated_string += &(line.unwrap() + "\n"); 
    }
    let translated_string = translate(non_translated_string.clone());
    let striped_string = stripstring(non_translated_string.clone());
    let orginized_words = &striped_string.split_whitespace().collect::<Counter<_>>().most_common(); // Organizing data into a Counter collection
    let mut translated_orginized_words = String::new();
    for i in orginized_words
    {
        translated_orginized_words += &(i.0.to_owned() + "\n"); 
    }
     
     translated_orginized_words = translate(translated_orginized_words);
     let vector_translated_orginized_words: Vec<_> = translated_orginized_words.split("\n").collect();
    for (ele, ele2) in orginized_words.iter().zip(vector_translated_orginized_words.iter())
        {  
            write!(output_file, "{:?} " ,ele).expect("Could not write to file");
            write!(output_file, "{:?}\n" ,ele2).expect("Could not write to file");
            
        }
    let unique_words = orginized_words.len();
    let total_words = striped_string.split_whitespace().count();
    write!(output_file, "Number of unique words: {}\n" ,unique_words).expect("Error");
    write!(output_file, "Number of total words: {}\n" ,total_words).expect("Error");
    let non_translated_vec: Vec<_> = non_translated_string.split("\n").collect();
    let translated_vec: Vec<_> = translated_string.split("\n").collect();
    
for (phrase, translated_phrase) in non_translated_vec.iter().zip(translated_vec.iter())
    {   if *phrase != ""
    {
        write!(output_file, "{} " ,phrase).expect("Error");
         write!(output_file, " ---- " ).expect("Error");
         write!(output_file, "{} " , translated_phrase).expect("Error");
         write!(output_file, "\n").expect("Error");
     }
    }
 
    println!("output.txt created");
  
}




