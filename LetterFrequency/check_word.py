from trigger_detection import get_triggers, aliases

while True:
    usr = input("Enter a word to check for aliases: ")
    print("\n".join(get_triggers(usr,aliases)))
