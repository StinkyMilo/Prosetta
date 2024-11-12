# Lit - Big Number Constructor

Meant for creating large numbers, this constructor allows the construction of a number one digit at a time. Until the next punctuation mark, all words are interpreted as numbers and involved in number construction. The number values are then modded by 10 and concatenated together, combining the last digit of each number to form a bigger number.

There are three ways the constructor interprets words as numbers. First, if the word is a variable name, it uses that variable's value. Next, if the word can be interpreted as a number literal (see number literals below), it uses that number literal's value. Otherwise, it uses the length of the word. See examples below to see each in action.

## Arguments

```w1, w2, ... (words)```

## Example
<editor :code='`
Lit Example
by Milo Jacobs\n
was var two.
was then lit var thirteen revolution..
pri then.
`' 
:code-wordier="`
Lit Example
by Milo Jacobs\n
Was var two?
If it was then literally var thirteen revolution!
Print then!
`"
output-method='console'></editor>