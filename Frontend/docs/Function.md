# Fun - Function

Defines a function. After defining it, all words until the next punctuation are considered parameters of the function. After that, it takes any number of statements to execute in sequence. The function can be called later with any word that substring matches the function name, and then specifying the correct number of arguments as expressions.

## Arguments

```[arg_1, ... arg_∞] (words) [CLOSING PUNCTUATION], [st_1, ... st_∞] (statements)```

## Example
<editor :code='`
Function Example
by Milo Jacobs and Lisa Perfunctory\n
fun factorial man.
	whe par man 1. ret 1..
	ret tim man factorial sub man 1...
was value factorial 5..
pri value.
`' 
:code-wordier="`
Function Example
by Milo Jacobs and Lisa Perfunctory\n
Fun factorial, man!
	When you compare man and one, do a thing. Return one answer!
	Return time, man. Factorials submit, man, one final time...
Was value a factorial or five rats in a trenchcoat?
Print a value, if you want!
`"
output-method='console'></editor>