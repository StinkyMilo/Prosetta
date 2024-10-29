# Prosetta
 A Poetic, Graphical Esolang

When coding in Prosetta, your code will be a "hidden message" within a larger poem or other piece of writing. This is accomplished by looking for specific aliases within your code that trigger commands. An alias is a subsequence of letters that triggers a command, and a command is a function or other standard programming language feature. When the parser reaches a word that contains an alias, the corresponding command is triggered, and the next part of the poem will be used to provide arguments to that command. Every piece of text that doesn't trigger an alias or otherwise parse will be ignored. Additionally, you will never receive a syntax error.

If something doesn't parse correctly, the language will also ignore it. Though the language does not enforce this process, the general way to write a program in Prosetta is to start by getting the code working with minimal aliases and nothing ignored, then modifying the code to add new letters and punctuation such that the code becomes a coherent piece of writing as well as working code.

<editor :code="`
tur 45.
pen 15.
rec 100.
`" 
:code-wordier=null
output-method='canvas'></editor>