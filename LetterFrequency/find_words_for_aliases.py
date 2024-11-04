import trigger_detection as td
import json

aliases = td.aliases

words = [i.split(",")[0] for i in open("unigram_freq.csv", "r").read().split("\n")][1:]

with open("./words_alpha.txt", "r") as file:
    good_dict = set([s.strip() for s in file.readlines()])

trigger_data = {}
for alias in aliases:
    trigger_data[alias] = []

for word in words:
    if word not in good_dict:
        continue
    result = td.get_single_trigger(word, aliases)
    if result is not None:
        trigger_data[result].append(word)

open("words_for_aliases.txt", "w").write(json.dumps(trigger_data, indent=4))
