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
def reset_interp(commands):
    global interp
    interp = []
    for com in commands:
        interp.append(Progress(com))
        
letters = 'abcdefghijklmnopqrstuvwxyz'

def triggers(word,commands,only_one=False):
    reset_interp(commands)
    triggers = []
    for letter in word:
        for progress in interp:
            if progress.letter == letter and progress.inc():
                triggers.append(progress.alias)
                reset_interp(commands)
                if only_one:
                    return progress.alias
    if only_one:
        return None
    return triggers

