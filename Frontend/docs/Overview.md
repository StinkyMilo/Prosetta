# Using the Language

When coding in Prosetta, your code will be a "hidden message" within a larger poem or other piece of writing. This is accomplished by looking for specific aliases within your code that trigger commands. An alias is a subsequence of letters that triggers a command, and a command is a function or other standard programming language feature. When the parser reaches a word that contains an alias, the corresponding command is triggered, and the next part of the poem will be used to provide arguments to that command. Every piece of text that doesn't trigger an alias or otherwise parse will be ignored. Additionally, you will never receive a syntax error. If something doesn't parse correctly, the language will also ignore it.

Though the language does not enforce this process, the general way to write a program in Prosetta is to start by getting the code working with minimal aliases and nothing ignored, then modifying the code to add new letters and punctuation such that the code becomes a coherent piece of writing as well as working code. Using this process, you can make a poem about nature that parses to an image of a flower or a poem about a calculator that prints out some calculations, or you can hide an animation of a rotating eye in what seems to be an introduction to a programming language (not this introduction, sadly, but we'll link to one here eventually!) 

# How the Parser Works

The parser takes input in Rust and converts it to an Abstract Syntax Tree which it then converts to JavaScript code which then runs on the web interface. When unsure about things like variable scoping, data type issues, truthiness, etc., default to JavaScript's documentation!

# The Alias System

Aliases are triggered by words that contain them as a subsequence. This is different from a substring in that, although all letters of the alias must be present in the correct order, the subsequence may be interrupted by other letters. For example, the word "wizards" triggers the alias "was", because it contains w then a then s, even though there are other letters between them. 

Each word triggers only one alias. If a word contains multiple aliases as subsequences, whichever alias is triggered earlier in the word takes effect. If both aliases are triggered on the same letter, the one with the lower distance between the first and last letter of the alias takes effect. If both are the same, the alias triggered is the one with a lower sum of all letter indices in the alias, or in more qualitative terms, whichever alias is more concentrated towards the start of the word.

All aliases are essentially functions. Similar to Lisp, everything is written in prefix notation, meaning that code traditionally written as "2+3" would instead be written in Prosetta as "add 2 3.". Commands are implicitly opened when their alias is triggered and are closed by punctuation. Different punctuation closes different numbers of commands so as to facilitate closing of large amounts of punctuation at once.

# Types in Prosetta

Prosetta has a mix of strong and loose typing. Variables, like in JS, can be any type. Therefore, when using variables in functions and aliases, it's important to check that they are the proper type. 

Aliases have specific types they are looking for, in a specific order. For example, the `fil` alias will look for a color, a single number, or 3 numbers. If you provide two numbers and a color, the alias will not close until it receives a third number. However, if you provide two numbers and a variable, it will close even if that variable happens to be a color. This will not fill with the color you expect, so it should be avoided!


