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

open("parts_of_speech_wordnet.json","w").write(json.dumps(word_dict))