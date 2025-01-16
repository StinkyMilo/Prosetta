# Quickstart - Part 1

Prosetta is an esoteric programming language in which code sounds like poetry. A common goal of Prosetta programs is to hide the code you write within a poem or piece of prose. This is accomplished by looking for specific aliases within your code that trigger commands. An alias is a subsequence of letters that triggers a command, and a command is a function or other standard programming language feature. When the parser reaches a word that contains an alias, the corresponding command is triggered, and the next part of the poem will be used to provide arguments to that command. Every piece of text that doesn't trigger an alias or otherwise parse will be ignored. 

This tutorial will guide you through getting started with Prosetta. It will assume you have some programming experience, but it will not require that you know any one language (though JavaScript will be particularly helpful).

## Using the Editor

You can see a version of the Prosetta editor below. If you ever want more space, you can view the main editor [here](https://stinkymilo.github.io/Prosetta/Frontend/). 

<editor :code="`
Editor Example
by Milo Jacobs and John Graphton\n
arc 50.
fil red.
`" 
:code-wordier="`
Editor Example
by Milo Jacobs and John Graphton\n
You must march fifty meters.
Filthy red balloon for you.
`"
output-method='canvas'></editor>

### Minimal vs. Wordier
The editor in the documentation pages will have both a "minimal" and "wordier" version. You can click the button at the top to switch between those two modes. The minimal version shows you just the aliases; in other words, only the parts of the program that the compiler pays attention to. The wordier version gives an idea of how you could turn the minimal version into a program.

### Canvas vs. JavaScript
Prosetta's "compiler" is actually a "transpiler", meaning it converts your code to JavaScript then runs the JavaScript code. Switching the toggle at the top of the editor will let you view that code, which is useful for debugging purposes.

### Word Suggestions
If you hover over an alias with your mouse, you can view suggestions for words that you can use in place of the current word that will trigger the same command. These words are in order of their frequency of use, so more common words will appear first. You can click the buttons on the popup to filter by different parts of speech. If you click any of the words, the editor will insert that word for you, and if you click the name of the alias (highlighted in green) you can view the documentation page for that alias. These suggestions will be very useful for converting "minimal" programs to "wordier" ones.