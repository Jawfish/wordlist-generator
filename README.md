# wordlist-generator
Generate a list of words that only contain the specified letters from a given dictionary. Intended to create targeted lists of words for practicing certain sequences of characters on [Monkeytype](https://monkeytype.com/).

## Usage

1. In the main directory, place a text file with a list of words, one per line, to filter (named `dictionary.txt`, configurable in `config.ini`).
2. In `config.ini`, set `Sequence` to be an array of characters you want to filter for. Example: `Sequence = ['a', 'b', 'c']` will result in an `output.txt` file with only words from `dictionary.txt` containing some combination of `a`, `b`, and/or `c`.
3. In `config.ini`, set `MinLength` to be the minimum size of the words you want to appear in `output.txt`.
4. Run `python main.py` and your filtered words will saved to `output.txt`.
5. If you are using this to make a custom dictionary for [MonkeyType](https://monkeytype.com/), do the following on the main page:
   1. Click `custom` in the top right
   2. Click `change` below `custom`
   3. Paste the contents of `output.txt` into the text field
   4. Select your desired options below that and click `ok`