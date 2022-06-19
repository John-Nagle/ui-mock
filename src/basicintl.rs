//  basicintl.rs -- Very basic internationalization system.
//
//  Suitable for use with "egui", which needs a lookup on every frame.
//
//  Animats
//  June, 2022
//
use anyhow::{anyhow, Context, Error};
use oxilangtag::LanguageTag;
use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::Read;

//  If no locale info is available, pick one of these, in order, as available.
const IMPERIAL_LANGUAGES: [&str; 3] = ["en", "cn", "ru"]; // should support at least one of these. May support more

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
macro_rules! t {
    ($s:expr,$dict:expr) => {
        // macro expands this
        {
            static MSG: OnceCell<&str> = OnceCell::new();
            let s: &str = MSG.get_or_init(|| {
                println!("Did Lookup of {}", $s); // ***TEMP***
                $dict.translate($s) // first time only
            });
            s
        }
    };
}

/// Format of the translations dictionary for a single language
type TranslationDictionary = HashMap<&'static str, &'static str>;
/// Format of the translations file, in JSON
type TranslationFile = HashMap<String, HashMap<String, String>>; // File contents { "key" : {"lang", "key in lang" }}

/// Language dictionary. Constructed from a JSON file.
pub struct Dictionary {
    translations: TranslationDictionary, // translations for chosen language
}

impl Dictionary {
    /// Create the dictionary for one language.
    pub fn new(files: &[&str], langid: &str) -> Result<Dictionary, Error> {
        let mut translations = HashMap::new();
        //  Add translations from all JSON files
        let mut languages = HashSet::new(); // list of languages
        for file in files {
            let translation_file = Self::read_translation_file(file)
                .with_context(|| format!("Translation file: \"{}\"", file))?;
            Self::validate_translation_file(&translation_file, &mut languages)
                .with_context(|| format!("Translation file: \"{}\"", file))?;
            Self::add_translations(&mut translations, &translation_file, langid)?;
            log::info!("Loaded translations from {}", file); // note translations loaded
        }
        Ok(Dictionary { translations })
    }

    /// Get list of available languages.
    //  Reading the first entry will tell us this, because all entries have to match.
    pub fn get_language_list(files: &[&str]) -> Result<HashSet<String>, Error> {
        if files.is_empty() {
            return Ok(HashSet::new()); // empty list, no translations available
        }
        let file = files[0]; // we have at least one
        let translation_file = Self::read_translation_file(file)
            .with_context(|| format!("Translation file: \"{}\"", file))?;
        for (_, v) in translation_file {
            return Ok(v.iter().map(|(k, _)| k.clone()).collect()); // unordered list of available translations
        }
        return Ok(HashSet::new()); // empty list, no translations available
    }

    // Make static string, which we must do so we can create strings which can be memoized in static variables.
    fn string_to_static_str(s: String) -> &'static str {
        Box::leak(s.into_boxed_str())
    }

    /// Read the JSON translation file tnto a Translationfile structure.
    fn read_translation_file(filename: &str) -> Result<TranslationFile, Error> {
        //  Read one translations file
        let file = File::open(filename)
            .with_context(|| anyhow!("Failed to open the translations file: {}", filename))?;
        let mut reader = std::io::BufReader::new(file);
        let mut content = String::new();
        reader
            .read_to_string(&mut content)
            .context("Failed to read the translations file")?;
        serde_json::from_str(&content).context("Failed to parse translations file")
    }

    /// Add translations from a JSON file.
    /// Add only for one language, which cannot be changed once initialized.
    fn add_translations(
        res: &mut TranslationDictionary,
        translation_file: &TranslationFile,
        langid: &str,
    ) -> Result<(), Error> {
        //  Select desired language from translations file
        for (key, value) in translation_file {
            println!("Key: {}, Value: {:?}", key, value); // ***TEMP***
            if let Some(v) = value.get(langid) {
                //  We have a translation for this key for this language
                res.insert(
                    Self::string_to_static_str(key.to_string()),
                    Self::string_to_static_str(v.to_string()),
                ); // add to translations
            } else {
                //  Translation file needs repair
                return Err(anyhow!(
                    "No translation for key {}, language {}",
                    key,
                    langid
                ));
            };
        }
        Ok(())
    }

    /// Validate entire translation file for having a translation for every language mentioned
    fn validate_translation_file(
        res: &TranslationFile,
        languages: &mut HashSet<String>,
    ) -> Result<(), Error> {
        for (key, value) in res {
            Self::validate_translation_set(&key, &value, languages)?; // check that all translations are present
        }
        Ok(())
    }

    /// Check that each translation has all the languages
    fn validate_translation_set(
        key: &str,
        value: &HashMap<String, String>,
        languages: &mut HashSet<String>,
    ) -> Result<(), Error> {
        let this_set: HashSet<String> = value.iter().map(|(k, _v)| k.clone()).collect(); // all the languages
                                                                                         //  Language list from first language becomes the master
        if languages.is_empty() {
            *languages = this_set.clone();
        }
        if this_set != *languages {
            let missing = languages.difference(&this_set);
            return Err(anyhow!(
                "Translation dictionary is missing a translation to {:?} for \"{}\"",
                missing,
                key
            ));
        }
        Ok(())
    }

    //  Lookup, only done once per t! macro expansion
    pub fn translate<'a>(&self, s: &str) -> &'static str {
        if let Some(st) = self.translations.get(s) {
            st
        } else {
            log::error!("No translation is available for \"{}\"", s); // non-fatal error.
            Self::string_to_static_str(s.to_string()) // use the key as the result
        }
    }

    /// Get translation dictionary
    pub fn get_translation(locale_files: &[&str]) -> Result<Dictionary, Error> {
        fn pick_default_language(available: &HashSet<String>) -> Result<String, Error> {
            for lang in IMPERIAL_LANGUAGES.iter() {
                if available.contains(&lang.to_string()) {
                    return Ok(lang.to_string());
                }
            }
            //  No major languages available. Pick at random from available translations.
            //  Probably means someone substituted an unusual translations file that
            //  contains none of the major languages and does not match the system locale.
            for lang in available {
                log::error!(
                    "No default language choices available. Picking \"{}\"",
                    lang
                );
                return Ok(lang.clone());
            }
            //  We give up.
            Err(anyhow!("No language translations are available"))
        }
        //  Get list of languages for which we have a translation
        let lang_list = Dictionary::get_language_list(locale_files)?; // get list of supported languages.
        let locale_opt = sys_locale::get_locale(); // system locale
        log::info!(
            "System locale: {:?}.  Available language translations: {:?}",
            locale_opt,
            lang_list
        );
        let lang_tag = if let Some(locale) = locale_opt {
            let locale = locale.replace("_", "-"); // Workaround for https://github.com/1Password/sys-locale/issues/3
            let language_tag = LanguageTag::parse(locale)?; // system locale is garbled if this doesn't parse.
            let tag = language_tag.primary_language(); // two-letter tag
            if lang_list.contains(tag) {
                // if matches locale
                tag.to_string() // use it
            } else {
                pick_default_language(&lang_list)? // pick some default
            }
        } else {
            log::error!("System did not provide a locale.");
            pick_default_language(&lang_list)? // pick some default
        };
        Self::new(locale_files, &lang_tag) // build the translations dictionary
    }
}

#[test]
fn test_translation() {
    use once_cell::sync::OnceCell;
    //  Initialize the dictionary
    let locale_file = concat!(env!["CARGO_MANIFEST_DIR"], "/src/locales/menus.json"); // test only
    let dictionary: Dictionary = Dictionary::new(&[locale_file], "fr").unwrap(); // build translations for "fr".
                                                                                 //  Demonstrate that it only does the lookup once
    for _ in 1..5 {
        println!("{} => {}", "menu.file", t!("menu.file", &dictionary));
    }
    assert_eq!("Fichier", t!("menu.file", &dictionary)); // consistency check
}
