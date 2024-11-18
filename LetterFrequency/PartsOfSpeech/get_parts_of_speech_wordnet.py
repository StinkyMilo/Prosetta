import json

all_words = open("../words_alpha.txt","r").read().split("\n")
all_words_cross = set([i.split(",")[0] for i in open("../unigram_freq.csv", "r").read().split("\n")][1:])
all_words = [i for i in all_words if i in all_words_cross]

def get_words(file):
    initial_values = [i.split(" ")[4] for i in open(file,"r").read().split("\n")[29:-1]]
    return [i for i in initial_values if "_" not in i]

word_sets = {
    "noun":set(get_words("SpeechWordnet/data.noun")),
    "adjective":set(get_words("SpeechWordnet/data.adj")),
    "adverb":set(get_words("SpeechWordnet/data.adv")),
    "verb":set(get_words("SpeechWordnet/data.verb"))
}

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
    ],
    "other":[
        "other-categories/mostly-conjunctions.txt",
        "other-categories/mostly-interjections.txt",
        "other-categories/mostly-prepositions.txt"
    ]
}

def flatten(ls):
    output = []
    for ls2 in ls:
        for item in ls2:
            output.append(item)
    return output

word_sets_backup = {i:set(flatten([open("PartOfSpeechList/"+j,"r").read().split("\n") for j in part_sources[i]])) for i in part_sources}

for i in word_sets_backup:
    if i == "other":
        continue
    word_sets[i] = word_sets[i].union(word_sets_backup[i])

word_dict = {}

for word in all_words:
    pos = []
    for part in word_sets:
        if word in word_sets[part]:
            pos.append(part)
    if len(pos) == 0:
        print("Missed",word)
        word_dict[word] = ["other"]
    else:
        print("Found",word)
        word_dict[word] = pos

open("parts_of_speech_wordnet.json","w").write(json.dumps(word_dict,indent=4))