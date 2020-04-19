extern crate regex;
use std::io::prelude::*;
use std::fs::File;
use regex::Regex;

/*
The stripfile function will remove all unnecessary chars from the input file i.e  punctuation, digits and some spanish chars
It will also turn the input text all lowercase

*/
pub fn stripfile(mut input_file: File) ->  std::string::String
{   let mut contents = String::new();
    let strip_misc = Regex::new(r"[[:punct:][:digit:][¿¡\n]]").unwrap();
    input_file.read_to_string(&mut contents).unwrap();
    contents.make_ascii_lowercase();
    let contents = strip_misc.replace_all(&contents,"");
    contents.into_owned()

}