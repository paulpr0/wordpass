extern crate rand;

use std::fs::File;

use argh::{FromArgs};

use lib_wordpass::{get_word_list, get_default_filename, generate_password};
/// Features: word pass can have spaces, capitals at the start or for each word.
/// The min words and min length can be configured
/// The results can be printed or copied to the clipboard
/// Constant strings can pe pre or appended
/// A number can be included (fixed or variable),
/// as can a special char from a list (with sensible default)
///
/// It will use a config in ~/.config/wordpass/config if such a config exists
///
/// All options can be specified in config file or as cmd line options
///
///
#[derive(FromArgs, Clone)]
/// Read from stdin, chunk and write back out again.
/// After a set delay, write out anything which is queued
struct Opts {
    /// minimum number of words in the password. Default is 4
    #[argh(option, short='w', default = "4")]
    min_word_count: u8,

    /// minimum number of letters in the password. Default is 16. Does not include spaces.
    #[argh(option, short='l', default = "16")]
    min_letter_count: u8,

    /// use spaces
    #[argh(switch, short='S')]
    do_not_use_spaces: bool,

    /// initial capital letter (default false)
    #[argh(switch, short ='i')]
    initial_capital: bool,

    ///capitalise all words
    #[argh(switch, short='c')]
    use_caps: bool,

    ///word list file. Defaults to <config>/wordpass/words.txt if it exists
    /// and /usr/share/dict/words if not.
    #[argh(option, short='d')]
    dictionary_file: Option<String>,

    ///by default we filter out non-alpha characters, and words with uppercase letters in.
    /// Set this option to prevent any filtering and use the dictionary as is.
    #[argh(switch)]
    do_not_filter:bool

}



fn main() {
    let opts:Opts = argh::from_env();
    let filename;
    if opts.dictionary_file.is_some() {
        filename = opts.dictionary_file.unwrap();
    } else {
        filename= match get_default_filename() {
            Some(f) => f,
            None => {
                println!("Cannot find dictionary file at a default location ($XDG_CONFIG_HOME/wordpass/words.txt or /usr/share/dict/words). To specify a dictionary file as an argument, see --help");
                return;
            }

    }
    }
    let file = match  File::open(filename) {
        Err(e) => {println!("Unable to open file: {}",e);return;},
        Ok(f) => f
    };
    println!("{}",generate_password(get_word_list(file, !opts.do_not_filter), opts.min_letter_count, opts.min_word_count, opts.initial_capital, opts.use_caps, !opts.do_not_use_spaces));
    println!();
}
