use std::fs::File;
use std::{io, env};
use std::io::BufRead;
use std::path::Path;
use rand::Rng;
use std::fmt::Write;
use inflector::Inflector;

/// Generate a password using words selected at random from the list of words.
/// The resulting string will have at least *min_letter_count* letters (excluding spaces)
/// and at least *min_word_count* words.
/// If *initial_capital* is true, the first word will begin with a capital letter
/// If *use_caps* is true, all words will begin with a capital letter.
/// If *use_spaces* is true, words will be separated by spaces.
pub fn generate_password(words:Vec<String>, min_letter_count:u8, min_word_count:u8, initial_capital:bool, use_caps:bool, use_spaces:bool ) ->String{

    let mut char_count = 0;
    let mut rng = rand::thread_rng();
    let mut chosen_words = vec![];
    while char_count < min_letter_count as usize || chosen_words.len()<min_word_count as usize {
        let n: usize = rng.gen();
        let pos = n % words.len();
        chosen_words.push(words.get(pos).unwrap());
        char_count += chosen_words.last().unwrap().len();
    }
    let mut chosen_iter = chosen_words.iter();
    let mut res = String::new();
    if initial_capital || use_caps {
        write!(res, "{}", chosen_iter.next().unwrap().to_title_case()).expect("Unable to write data");
    } else {
        write!(res, "{}", chosen_iter.next().unwrap()).expect("Unable to write data");
    }
    while let Some(word) = chosen_iter.next() {
        if use_spaces {
            write!(res, " ").expect("Unable to write data");
        }
        if use_caps {
            write!(res, "{}", word.to_title_case()).expect("Unable to write data");
        } else {
            write!(res, "{}", word).expect("Unable to write data");
        }
    }
    res
}
/// Get a *Vec<String>* of words from a file.
/// If filter is true, words which are not lowercase ascii are filtered. This is helpful to filter
/// out words with apostrophes, proper nouns or names beginning with capital letters.
pub fn get_word_list(file:File, filter:bool) -> Vec<String> {
    let file_reader =  io::BufReader::new(file);
    let mut words = Vec::with_capacity(10_000);

    for l in file_reader.lines() {
        let line = l.unwrap();
        if !filter {
            words.push(line);
        } else {
            if !line.contains(|c:char|!c.is_ascii_lowercase()) {
                words.push(line);
            }
        }
    }
    words
}

///Try to get a reasonable default word list. This will sometimes be effective on modern linux systems
/// We look first in the user's config folder ($XDG_CONFIG_HOME) to see if there is a folder for wordpass.
/// If so, and it contains a file words.txt, we use that.
/// If not, we look for a system dictionary in */usr/share/dict/words*
/// Returns the filename for a dictionary if found, else None.
pub fn get_default_filename() -> Option<String> {

    let word_list_in_config =  env::var("XDG_CONFIG_HOME")
        .unwrap_or(env::var("HOME")
            .unwrap_or("~/".to_string())+".config")
        + "wordpass/words.txt";

    if Path::exists(Path::new(&word_list_in_config)) {
        return Some(word_list_in_config);
    }
    let default_dictionary = "/usr/share/dict/words".to_string();
    if Path::exists(Path::new(&default_dictionary)) {
        return Some(default_dictionary);
    }
    None
}

