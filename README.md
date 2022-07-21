# wordlist-generator

Generate a list of words that only contain the specified letters from a given dictionary. Intended to create targeted lists of words for practicing certain sequences of characters on [Monkeytype](https://monkeytype.com/).

## Usage

| Flag | Default              | Description                                                                                                                                      |
| ---- | -------------------- | ------------------------------------------------------------------------------------------------------------------------------------------------ |
| `-d` | `"dictionary.txt"`     | The file containing the words to filter, one per line.                                                                                           |
| `-l` | Required, no default | The characters you want to filter for. Example: `abc` will output words from the dictionary containing some combination of  `a`, `b`, and/or `c` |
| `-o` | `"output.txt"`         | The file to write the words to.                                                                                                                  |
| `-m` | `1`                  | The minimum size of words to filter for.                                                                                                         |

If you are using this to make a custom dictionary for [MonkeyType](https://monkeytype.com/), do the following on the main page:

1. Click `custom` in the top right
2. Click `change` below `custom`
3. Paste the contents of `output.txt` into the text field
4. Select your desired options below that and click `ok`
