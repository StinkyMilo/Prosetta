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

def triggers(word):
    reset_interp()
    triggers = []
    for letter in word:
        for progress in interp:
            if progress.letter == letter and progress.inc():
                triggers.append(progress.alias)
                reset_interp()
                # No further letters viewed if the command looks at the next line
                if '$' in commands[progress.alias]:
                    return triggers
                break
    return triggers

