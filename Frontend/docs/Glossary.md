# Important Vocabulary

## Words

Some aliases take word arguments. These arguments will always be the first of a list of arguments, and they will take the entirety of the next word. They are typically used as names of variables or functions. The command "was" for example, takes a word argument and one regular argument. The word argument is the name of the variable and the regular argument is its value. Variables and functions can be referenced in other locations by the same word, so if I define the value of "variable" to be two, all instances of the word "variable" will be interpreted as the number two. 

In future versions of Prosetta, accessing variable and function names will be more liberal. Rather than needing to type the exact same word, you will instead be able to type any word that contains that word as a substring (Not a subsequence. The letters have to be contiguous). 

## Statements vs. Expressions
	
In simple terms, an expression has a return value and a statement simply does something. In most languages, this distinction isn't important, but Prosetta will ignore any expression that is not part of a statement. Therefore lines simply doing addition will not parse. You need to output them with a print command or by setting a variable to the value.

## Subsequence Matching
	
See [The Alias System](Overview.md#the-alias-system).

## Substring Matching
	
Variables, functions, and ignored values all use substring matching. Unlike subsequence matching, all letters in the string must be in a single contiguous block in order for the string to match. This means that you can, for example, declare a function called "walk" and then the word "walked" will evaluate to that variable.

If multiple substrings are matched, the longer substring takes priority.