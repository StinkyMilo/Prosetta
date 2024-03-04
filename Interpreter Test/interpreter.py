commands = {
    'pr':'(print',
    'pi':'(circle',
    'h':')',
    'li':'(line',
    'eq':'(def $',
    'num':'(length $',
    'mu':'(*',
    'and':'(+'
}
class Progress:
    def __init__(self,alias):
        self.alias = alias
        self.letter = alias[0]
        self.num = 0
    def inc(self):
        self.num+=1
        if self.num >= len(self.alias):
            return True
        self.letter = self.alias[self.num]
        return False

interp = []
variables = set()
def reset_interp():
    global interp
    interp = []
    for com in commands:
        interp.append(Progress(com))

letters = 'abcdefghijklmnopqrstuvwxyz'

def interpret(text):
    reset_interp()
    output = []
    text=text.lower()
    text = text.split()
    text = [''.join([i for i in word if i in letters]) for word in text]
    # print(text)
    i=0
    while i < len(text):
        # print("Analyzing word " + text[i])
        if text[i] in variables:
            # print("Recognized variable")
            output.append(text[i])
        else:
            j = 0
            while j < len(text[i]):
                # print("Analyzing letter " + text[i][j])
                skipping=False
                for progress in interp:
                    if progress.letter == text[i][j] and progress.inc():
                        com = commands[progress.alias]
                        # TODO: A more generalized system
                        if progress.alias == 'eq':
                            # print("Adding variable " + text[i+1])
                            variables.add(text[i+1])
                        # TODO: Allow for multiple $ signs
                        if '$' in com:
                            output.append(com[:com.index("$")])
                            output.append("'"+text[i+1]+"'")
                            i+=1
                            skipping=True
                        else:
                            output.append(com)
                        reset_interp()
                        break
                if skipping:
                    break
                j+=1
        i+=1
    return ' '.join(output)

program = """
Equals inch innumerably. Rabbitfish hide in Hell.
Equations miles across amuse you as you inch, inch again, heating, heaving.
Equate furlongs to ambiguity; disencumber your heels. Inch farther, farther. 
Equip longer armour. For miles, you swing your pendulum as you head home, harrowed.
Pick your pace. Go longer,  longer,  yet longer. Take the road below you.
Point East. Only two miles longer, to furlongs more ahead.
Point West. And no longer do miles hang before you. But longer furlongs await ahead.
Like miles before and longer miles after, home remains away from view. And longer miles await ahead. And even longer miles stretch behind. 
"""

print(interpret(program))