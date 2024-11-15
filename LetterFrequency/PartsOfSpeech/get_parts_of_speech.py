all_words = open("../words_alpha.txt","r").read().split("\n")

part_sources = {
    "noun":[
        "nouns/mostly-nouns.txt",
        "nouns/mostly-nouns-ment.txt",
        "mostly-plural-nouns.txt"
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

"""
Go through words_alpha.txt and cross-reference with part_sources to generate. If it fits in none, add it to other. Otherwise add it to all categories it matches
Make a JS dictionary that, for each word, gives it a list of parts of speech.
When making the popup, go through each word and instead of just adding it, add it to the *correct* subsections.
"""