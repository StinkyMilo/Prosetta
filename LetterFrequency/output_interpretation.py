values = [[x[0], float(x[1])] for x in ([i.split(", ") for i in open("three_letters_over_unigrams.csv","r").read().split("\n")])]
values.sort(key=lambda x: x[1],reverse=True)
open("3_letters_sorted.csv","w").write("\n".join([", ".join([values[i][0], str(i), str(values[i][1])]) for i in range(0,len(values))]))