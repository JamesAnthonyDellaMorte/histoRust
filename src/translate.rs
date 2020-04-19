extern crate ureq;
use ureq::json;

pub fn translate(input_phrase:String) 
-> String {	

let resp = ureq::post("https://translation.googleapis.com/language/translate/v2?key=AIzaSyBhB3zceTYVekQ93GqzdVfQLYNhaRoon00")
           .set("Content-Type:", "application/json; charset=utf-8")
            .send_json(json!({
             "q": input_phrase,
             "target": "en",
             "format": "text"}));
            let translation = &resp.into_json().unwrap()["data"]["translations"][0]["translatedText"];
 	        String::from(translation.as_str().unwrap())
    //       let v: Value = serde_json::from_str(resp.into_json().unwrap()).unwrap();

        }