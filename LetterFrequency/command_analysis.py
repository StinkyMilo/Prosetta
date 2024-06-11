import trigger_detection as td
import matplotlib.pyplot as plt
import itertools as it
scores = {}
commands = []

num_letters = 3
freq_file = 'unigram_freq.csv'
output_file = 'output.txt'

for command in ["".join(i) for i in it.product(td.letters,repeat=3)]:
    commands.append(command)
    scores[command]=0
scores['none']=0

words = [i.split(',') for i in open(freq_file,'r').read().split('\n')][1:]

words_completed = 0

for item in words:
    word = item[0]
    freq = int(item[1])
    triggers = td.triggers(word,commands)
    for trigger in triggers:
        scores[trigger]+=freq
    if len(triggers)==0:
        scores['none']+=freq
    words_completed+=1
    if words_completed%250==0:
        print("Completed {0}/{1} words. Progress: {2}%".format(words_completed,len(words),(words_completed*100)//len(words)))

normal_factor = sum([int(i[1]) for i in words])

command_names = [i for i in scores]
command_scores = [scores[i] for i in scores]
open(output_file,'w').write("\n".join([""+i+", "+str(scores[i]/normal_factor) for i in scores]))
plt.pie(command_scores,labels=command_names)
plt.legend()
plt.show()