# Fre - Foreach

A for each loop. Takes a word to be assigned each value in a list sequentially, then has a body that repeats for each value in the provided list.

If `fre` is provided a number *n* rather than a list, it will use the list [0, 1, ... n-1].

## Arguments
```name (word), list (list), [st_1, ... st_∞] (statements)```

```name (word), range (number), [st_1, ... st_∞] (statements)```

## Example
<editor :code='`
Foreach Example
by Milo Jacobs and Miss Listerine\n
was value lis 1 2 3..
fre that value
	pri that.
.
`' 
:code-wordier="`
Foreach Example
by Milo Jacobs and Miss Listerine\n
was value a list of 1 2 3?
freedom that i crave, the value of Pride that I share!
`"
output-method='console'></editor>