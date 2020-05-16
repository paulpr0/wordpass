# wordpass
Generate (long) passwords using random words from a provided dictionary

A simple tool to generate passwords using words from a supplied dictionary. 
```
Usage: wordpass [-w <min-word-count>] [-l <min-letter-count>] [-S] [-i] [-c] [-d <dictionary-file>] [--do-not-filter]

Features: 
    Passwords can have spaces, a single capital at the start or for each word. 
    The min words and min length can be configured 
    It will use a dictionary in $XDG_CONFIG_HOME/wordpass/words.txt or /usr/share/dict/words if none is provided as an argument

Options:
  -w, --min-word-count
                    minimum number of words in the password. Default is 4
  -l, --min-letter-count
                    minimum number of letters in the password. Default is 16.
                    Does not include spaces.
  -S, --do-not-use-spaces
                    use spaces
  -i, --initial-capital
                    initial capital letter (default false)
  -c, --use-caps    capitalise all words
  -d, --dictionary-file
                    word list file. Defaults to <config>/wordpass/words.txt if
                    it exists and /usr/share/dict/words if not.
  --do-not-filter   by default we filter out non-alpha characters, and words
                    with uppercase letters in. Set this option to prevent any
                    filtering and use the dictionary as is.
  --help            display usage information
```
