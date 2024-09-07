import trigger_detection as td
import json

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
    "fre" "fun",
    "pri",
    "was",
    "sta",
    "not",
    "rec",
    "lin",
    "arc",
]

words = [i.split(",")[0] for i in open("unigram_freq.csv", "r").read().split("\n")][1:]

trigger_data = {}
for alias in aliases:
    trigger_data[alias] = []

for word in words:
    result = td.get_single_trigger(word, aliases)
    if result is not None:
        trigger_data[result].append(word)

open("words_for_aliases.txt", "w").write(json.dumps(trigger_data, indent=4))
