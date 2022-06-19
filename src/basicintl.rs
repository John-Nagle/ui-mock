//  basicintl.rs -- Very basic internationalization system.
//
//  Suitable for use with "egui", which needs a lookup on every frame.
//
//  Animats
//  June, 2022
//
use std::fs::File;
use std::io::Read;
use std::collections::{HashMap, HashSet};
use anyhow::{Error, Context, anyhow};

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
        let s: &str = MSG.get_or_init(|| {
            println!("Did Lookup of {}",$s); // ***TEMP*** 
            $dict.translate($s)    // first time only
        });
        s
    }
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
        //  Read and process one translations file
        let file = File::open(filename).with_context(|| anyhow!("Failed to open the translations file: {}", filename))?;
        let mut reader = std::io::BufReader::new(file);
        let mut content = String::new();
        reader
            .read_to_string(&mut content)
            .context("Failed to read the translations file")?;
        let res: HashMap<String, HashMap<String, String>> =
            serde_json::from_str(&content).context("Failed to parse translations file")?;
        let mut languages = HashSet::new();                 // list of languages
        for (key, value) in res {
            println!("Key: {}, Value: {:?}", key, value);   // ***TEMP***
            Self::validate_translation_set(&key, &value, &mut languages)?; // check that all translations are present
            if let Some(v) = value.get(langid) {
                //  We have a translation for this key for this language
                translations.insert(Self::string_to_static_str(key), Self::string_to_static_str(v.to_string()));    // add to translations
            } else {
                //  Translation file needs repair
                return Err(anyhow!("No translation for key {}, language {} in file {}", key, langid, filename));
            };
        }
        log::info!("Loaded translations from {}", filename);  // note translations loaded
        Ok(())
    }
    
    /// Check that each translation has all the languages
    fn validate_translation_set(key: &str, value: &HashMap<String, String>, languages: &mut HashSet<String>) -> Result<(), Error> {
        let this_set: HashSet<String> = value.iter().map(|(k,_v)| k.clone()).collect(); // all the languages
        //  Language list from first language becomes the master
        if languages.is_empty() {
            *languages = this_set.clone();
        }
        if this_set != *languages {
            let missing = languages.difference(&this_set);
            return Err(anyhow!("Translation dictionary is missing a translation to {:?} for \"{}\"", missing, key));
        }
        Ok(())
    }
    
    //  Lookup, only done once per t! macro expansion
    pub fn translate<'a>(&self, s: &str) -> &'static str {
        if let Some(st) =  self.translations.get(s) {
            st
        } else {
            log::error!("No translation is available for \"{}\"", s);    // non-fatal error.
            Self::string_to_static_str(s.to_string())          // use the key as the result
        }
    }
}
#[test]
fn test_translation() {
    use once_cell::sync::OnceCell;
    //  Initialize the dictionary
    let locale_file = concat!(env!["CARGO_MANIFEST_DIR"], "/src/locales/menus.json");    // test only
    let dictionary: Dictionary = Dictionary::new(&[locale_file],"fr").unwrap(); // build translations for "fr".
    //  Demonstrate that it only does the lookup once
    for _ in 1..5 {
        println!("{} => {}", "menu.file", t!("menu.file", &dictionary));
    }
    assert_eq!("Fichier", t!("menu.file", &dictionary)); // consistency check
}

