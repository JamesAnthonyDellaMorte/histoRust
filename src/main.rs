use std::io::prelude::*;
use std::fs::File;
mod stripfile;
mod translate;
use std::collections::HashMap;
use translate::translate;
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
    let mut s_words = String::new();
    for i in words
    {
        s_words += &(i.0.to_owned() + "\n"); 
    }
     
     s_words = translate(s_words);
     let values: Vec<_> = s_words.split("\n").collect();
     let mut key_map: HashMap<String, String> = HashMap::new();
    for (ele, ele2) in words.iter().zip(values.iter())
        {  // let word_en = translate(ele.0.to_string());
            write!(output_file, "{:?} " ,ele).expect("Could not write to file");
            write!(output_file, "{:?}\n" ,ele2).expect("Could not write to file");
            key_map.insert(ele.0.to_string(), ele2.to_string());
            
        }
    let unique_words = words.len();
    let total_words = contents.split_whitespace().count();
    write!(output_file, "Number of unique words: {}\n" ,unique_words).expect("Error");
    write!(output_file, "Number of total words: {}\n" ,total_words).expect("Error");
    //let words_en = translate(contents.clone());
  //  write!(output_file, "{:?}\n" , contents).expect("Error");
   // write!(output_file, "{:?}\n" ,words_en).expect("Error");
    let contents_vec: Vec<_> = contents.split(" ").collect();
    //let  words_en_vec: Vec<_> = words_en.split(" ").collect();
   // println!("{:?}", contents_vec);
    //println!("{:?}", words_en_vec);
    let mut count1 = 0;
  //  let mut count2  = 0;
 //   let mut idx = 0;
    let mut vector_buf = Vec::new();
    for word_1 in  contents_vec
    {
        write!(output_file, "{} " , word_1).expect("Error");
        count1 += 1;
        if word_1 != "".to_string()
        {
            vector_buf.push(word_1);
        } 
        if count1 == 10
        { 
            write!(output_file, " --------- " ).expect("Error");
             count1 = 0;
             for j in &vector_buf
             {
                 write!(output_file, "{} " , key_map.get(j.to_owned()).expect(j)).expect("Error");
                
             }
              write!(output_file, "\n ").expect("Error"); 
             vector_buf.clear()

             /*
            loop
             {

                write!(output_file, "{} " , words_en_vec[idx]).expect("Error"); 
                count2 += 1;
                idx += 1;
                
                if count2 == 10
                {
                    write!(output_file, "\n ").expect("Error"); 
                    count2 = 0;
                    break;
                }
                */
             }
        }

    
    println!("output.txt created");
  //  println!("{:?}\n", words_en, );
    //  write!(p, "Percent of words needed to know to understand 100% of text: {}" ,(unique_words as f64).div(total_words as f64)).expect("Error");
}




