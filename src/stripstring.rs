extern crate regex;
use regex::Regex;

/*
The stripfile function will remove all unnecessary chars from the input file i.e  punctuation, digits and some spanish chars
It will also turn the input text all lowercase

*/
pub fn stripstring(mut input_str: String) -> std::string::String {
    let strip_misc = Regex::new(r"[[:punct:][:digit:][\r¿¡\n]]").unwrap();
    input_str.make_ascii_lowercase();
    let contents = strip_misc.replace_all(&input_str, "");
    contents.into_owned()
}
