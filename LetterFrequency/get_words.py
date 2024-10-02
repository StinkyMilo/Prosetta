import json
import os

with open("./words_for_aliases.txt", "r") as file:
    keywords = json.load(file)

with open("./words_alpha.txt", "r") as file:
    good_dict = set([s.strip() for s in file.readlines()])

for alias in keywords:
    curr = []
    for word in keywords[alias]:
        if word in good_dict:
            curr.append(word)
    keywords[alias] = curr

del good_dict

last_keyword = ""
last_index = 0

while True:
    usr = input("Keyword for word list (or enter n to print next page): ")
    if usr in keywords:
        last_keyword = usr
        last_index = 0
        size = os.get_terminal_size()
        print("\n".join(keywords[usr][: size.lines - 3]))
        print("*" * size.columns)
    elif usr == "n" and last_keyword:
        size = os.get_terminal_size()
        last_index += size.lines - 3
        print("\n".join(keywords[last_keyword][last_index : last_index + size.lines - 3]))
        print("*" * size.columns)
    else:
        print(f'"{usr}" is not a valid keyword.')
