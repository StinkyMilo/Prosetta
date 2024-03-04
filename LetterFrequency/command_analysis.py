import trigger_detection as td
import matplotlib.pyplot as plt
scores = {}
for command in td.commands:
    scores[command]=0
scores['none']=0

words = [i.split(',') for i in open('unigram_freq.csv','r').read().split('\n')][1:]

for item in words:
    word = item[0]
    freq = int(item[1])
    triggers = td.triggers(word)
    for trigger in triggers:
        scores[trigger]+=freq
    if len(triggers)==0:
        scores['none']+=freq

normal_factor = 10000000

command_names = [i for i in scores]
command_scores = [scores[i] for i in scores]
open('output.txt','w').write("\n".join([""+i+", "+str(scores[i]//normal_factor) for i in scores]))
plt.pie(command_scores,labels=command_names)
plt.show()