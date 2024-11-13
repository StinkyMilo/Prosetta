word_set = set(open("words_alpha.txt","r").read().split("\n"))
words_in_order = [i.split(",")[0] for i in open("unigram_freq.csv", "r").read().split("\n")][1:]

all_words = [i for i in words_in_order if i in word_set]

open("words.js","w").write(
    "export const allWords = [" +
    ",\n".join(["\"" + i + "\"" for i in all_words])
    + "];"
)