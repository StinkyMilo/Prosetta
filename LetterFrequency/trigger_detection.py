letters = "abcdefghijklmnopqrstuvwxyz"

class Progress:
    def __init__(self, alias):
        self.alias: str = alias
        self._num: int = 0
        self.alias_start: int = -1
        self.alias_end: int = -1
        self.total_location_value: int = 0

    @property
    def letter(self) -> str:
        if self._num == len(self.alias):
            return ""
        return self.alias[self._num]

    def inc(self,index):
        if self._num < len(self.alias):
            if self._num == 0:
                self.alias_start = index
            self.total_location_value += index
            self._num += 1
        if self._num == len(self.alias):
            self.alias_end = index
            return True
        return False


def get_triggers(word: str, commands: list[str]) -> list[str]:
    interp = [Progress(command) for command in commands]
    triggers: list[str] = []
    i = 0
    for letter in word:
        for progress in interp:
            if progress.letter == letter and progress.inc(i):
                triggers.append(progress.alias)
        i+=1
    return triggers


# Will return the best trigger
def get_single_trigger(word: str, commands: list[str]) -> None | str:
    interp = [Progress(command) for command in commands]
    triggers: list[str] = []
    i = 0
    for letter in word:
        for progress in interp:
            if progress.letter == letter and progress.inc(i):
                triggers.append(progress)
        i+=1
    # print([i.alias for i in triggers])
    if len(triggers) == 1:
        return triggers[0].alias
    elif len(triggers) > 1:
        first_match_pos = min(triggers,key=lambda x: x.alias_end).alias_end
        triggers = [trigger for trigger in triggers if trigger.alias_end == first_match_pos]
        # print([i.alias for i in triggers])
        if len(triggers) == 1:
            return triggers[0].alias
        elif len(triggers) > 1:
            length_value_progress = min(triggers,key=lambda x: x.alias_end - x.alias_start)
            length_value = length_value_progress.alias_end - length_value_progress.alias_start
            triggers = [trigger for trigger in triggers if trigger.alias_end - trigger.alias_start == length_value]
            # print([i.alias for i in triggers])
            if len(triggers) == 1:
                return triggers[0].alias
            elif len(triggers) > 1:
                min_location_value = min(triggers,key=lambda x: x.total_location_value).total_location_value
                triggers = [trigger for trigger in triggers if trigger.total_location_value == min_location_value]
                # print([i.alias for i in triggers])
                if len(triggers) > 1:
                    print("Error: Triggers length still >1 at last step")
                elif len(triggers) == 1:
                    return triggers[0].alias
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
    "bez",
    "fra",
    "ran",
    "sin",
    "cos",
    "tan",
    "flo",
    "sta",
    "pol",
    "tri",
    "hea",
    "roc",
    "kir"
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
        # print("\n".join(get_triggers(args.check_word, aliases)))
        print(get_single_trigger(args.check_word,aliases))
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
