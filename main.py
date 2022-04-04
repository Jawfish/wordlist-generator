import configparser

config = configparser.ConfigParser()
config.read("config.ini")
sequence = config["DEFAULT"]["Sequence"]
input_filename = config["DEFAULT"]["InputFile"]
output_filename = config["DEFAULT"]["OutputFile"]
minimum_length = int(config["DEFAULT"]["MinLength"])

# turn words from file into a list
def get_words_from_file(file_name: str) -> list:
    with open(file_name, "r") as file:
        return [word.strip() for word in file.readlines()]


# return words from a given list that are composed of characters from a given sequence
def get_words_from_dictionary(
    dictionary: list, letter_sequence: list, min_length: int = 0
) -> list:
    def word_is_composed_of_sequence(word: str) -> bool:
        return all(ch in letter_sequence for ch in word)

    return [
        word
        for word in dictionary
        if word_is_composed_of_sequence(word) and len(word) >= min_length
    ]


# save words to file
def save_words_to_file(words: list, file_name: str) -> None:
    with open(file_name, "w") as file:
        for word in words:
            file.write(word + " ")


# run the program
save_words_to_file(
    get_words_from_dictionary(
        get_words_from_file("dictionary.txt"), sequence, minimum_length
    ),
    "output.txt",
)
