//  basicintl.rs -- Very basic internationalization system.
//
//  Suitable for use with "egui", which needs a lookup on every frame.
//
//  Animats
//  June, 2022
//
//  Usage:
//
//  For each item to be translated, write
//
//      t!("key", lang)
//
//  which will return a static string with the translation of "key".
//  This is a simple word lookup only. There is no substitution. 

use std::collections::HashMap;
use once_cell::sync::OnceCell;

//  The dictionary - just a hash table here.
type Dictionary<'a> = HashMap<&'a str, &'static str>;

//  Lookup, only done once per t! macro expansion
fn translate<'a>(s: &str, dict: &'a Dictionary) -> &'static str {
    dict.get(s).unwrap()
}

//  Translate with memoization
macro_rules! t{
    ($s:expr,$dict:expr)=>{
 // macro expands this
    {   static MSG: OnceCell<&str> = OnceCell::new();
        MSG.get_or_init(|| {
            println!("Did Lookup"); // ***TEMP*** 
            translate($s, $dict)    // first time only
        }
    )}
    }
}

fn string_to_static_str(s: String) -> &'static str {
    Box::leak(s.into_boxed_str())
}

fn main() {
    //  Initialize the dictionary
    let mut dictionary: Dictionary = HashMap::new();
    dictionary.insert("Hello", "Allo");
    dictionary.insert("Bye", "Au revoir");
    dictionary.insert("Go", "Allez");
    let stop = "Arrêt".to_string();
    dictionary.insert("Stop", string_to_static_str(stop));
    //  Demonstrate that it only does the lookup once
    for _ in 1..5 {
        println!("{} => {}", "Hello", t!("Hello", &dictionary));
        println!("{} => {}", "Hello", t!("Hello", &dictionary));
        println!("{} => {}", "Go", t!("Go", &dictionary));
        println!("{} => {}", "Stop", t!("Stop", &dictionary));
    }
}
