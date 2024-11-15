all_words = open("../words_alpha.txt","r").read().split("\n")

part_sources = {
    "noun":[
        "nouns/mostly-nouns.txt",
        "nouns/mostly-nouns-ment.txt",
        "nouns/mostly-plural-nouns.txt"
    ],
    "verb":[
        "verbs/mostly-verbs-infinitive.txt",
        "verbs/mostly-verbs-past-tense.txt",
        "verbs/mostly-verbs-present-tense.txt",
        "verbs/transitive-past-tense.txt",
        "verbs/transitive-present-tense.txt"
    ],
    "adjective":[
        "other-categories/mostly-adjectives.txt",
    ],
    "adverb":[
        "other-categories/ly-adverbs.txt",
        "other-categories/mostly-adverbs.txt"
    ]
}

def flatten(ls):
    output = []
    for ls2 in ls:
        for item in ls2:
            output.append(item)
    return output

word_sets = {i:set(flatten([open("PartOfSpeechList/"+j,"r").read().split("\n") for j in part_sources[i]])) for i in part_sources}

all_words = open("../words_alpha.txt","r").read().split("\n")

"""
Go through words_alpha.txt and cross-reference with part_sources to generate. If it fits in none, add it to other. Otherwise add it to all categories it matches
Make a JS dictionary that, for each word, gives it a list of parts of speech.
When making the popup, go through each word and instead of just adding it, add it to the *correct* subsections.
"""

word_dict = {}

for word in all_words:
    pos = []
    for part in word_sets:
        if word in word_sets[part]:
            pos.append(part)
    