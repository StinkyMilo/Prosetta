letters = "abcdefghijklmnopqrstuvwxyz"


class Progress:
    def __init__(self, alias):
        self.alias: str = alias
        self._num: int = 0

    @property
    def letter(self) -> str:
        if self._num == len(self.alias):
            return ""
        return self.alias[self._num]

    def inc(self):
        if self._num < len(self.alias):
            self._num += 1
        return self._num == len(self.alias)


def get_triggers(word: str, commands: list[str]) -> list[str]:
    interp = [Progress(command) for command in commands]
    triggers: list[str] = []
    for letter in word:
        for progress in interp:
            if progress.letter == letter and progress.inc():
                triggers.append(progress.alias)
    return triggers


def get_single_trigger(word: str, commands: list[str]) -> None | str:
    interp = [Progress(command) for command in commands]
    for letter in word:
        for progress in interp:
            if progress.letter == letter and progress.inc():
                return progress.alias
    return None


# from `cat words_for_aliases.txt| jq keys`
aliases = [
    "int",
    "lit",
    "add",
    "ide",
    "sub",
    "log",
    "exp",
    "mod",
    "tim",
    "lis",
    "ind",
    "app",
    "rep",
    "del",
    "whe",
    "les",
    "mor",
    "als",
    "oth",
    "par",
    "inv",
    "whi",
    "fre",
    "fun",
    "pri",
    "was",
    "sta",
    "not",
    "rec",
    "lin",
    "arc",
    "col",
    "els",
    "sto",
    "fil",
    "mov",
    "pen",
    "tur",
    "ret",
    "fin",
    "cou",
    "bez"
]

if __name__ == "__main__":
    import argparse

    parser = argparse.ArgumentParser()
    parser.add_argument(
        "-w", "--check-word", type=str, help="The word to test for triggers"
    )
    parser.add_argument(
        "-k",
        "--get-keyword",
        type=str,
        help="The keyword to return a list of words for",
    )
    args = parser.parse_args()
    if args.check_word:
        print("\n".join(get_triggers(args.check_word, aliases)))
    elif args.get_keyword:
        import json

        with open("./words_for_aliases.txt", "r") as file:
            keywords = json.load(file)
        if args.get_keyword in keywords:
            print("\n".join(keywords[args.get_keyword]))
        else:
            print(f'"{args.get_keyword}" is not a valid keyword.')
    else:
        print("\n".join(aliases))
