//  basicintl.rs -- Very basic internationalization system.
//
//  Suitable for use with "egui", which needs a lookup on every frame.
//
//  Animats
//  June, 2022
//
use std::collections::HashMap;
use anyhow::{Error};

///#  Translate with memoization
//
//  For each item to be translated, write
//
//      t!("key", lang)
//
//  which will return a reference to a static string with the translation of "key".
//  This is a simple word lookup only. There is no substitution.
//  Translations cannot be changed after first use.
#[macro_export]
macro_rules! t{
    ($s:expr,$dict:expr)=>{
    // macro expands this
    {   static MSG: OnceCell<&str> = OnceCell::new();
        MSG.get_or_init(|| {
            println!("Did Lookup of {}",$s); // ***TEMP*** 
            $dict.translate($s)    // first time only
        }
    )}
    }
}

/// Language dictionary. Constructed from a JSON file.
pub struct Dictionary{
    translations: HashMap<&'static str, &'static str>    // translations for chosen language
}

impl Dictionary {
    pub fn new(files: &[&str], langid: &str) -> Result<Dictionary, Error> {
        let mut translations = HashMap::new();      
        //  Add translations from all JSON files
        for file in files {
            Self::add_translations(&mut translations, file, langid)?;
        }   
        Ok(Dictionary {translations })
    }
    
    // Make static string, which we must do so we can
    // create strings that can be memoized
    fn string_to_static_str(s: String) -> &'static str {
        Box::leak(s.into_boxed_str())
    }
    
    /// Add translations from a JSON file.
    /// Add only for one language, which cannot be changed once initialized.
    fn add_translations(translations: &mut HashMap<&'static str, &'static str>, filename: &str, langid: &str) -> Result<(), Error> {
        //  ***MORE***
        Ok(())
    }
    
    //  Lookup, only done once per t! macro expansion
    pub fn translate<'a>(&self, s: &str) -> &'static str {
        self.translations.get(s).unwrap()
    }
}
#[test]
fn test_translation() {
    use once_cell::sync::OnceCell;
    //  Initialize the dictionary
    let mut dictionary: Dictionary = Dictionary::new(&[],"fr").unwrap();
    let s: &str = t!("Hello", dictionary);
    /*
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
    */
}

