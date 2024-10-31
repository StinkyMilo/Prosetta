# Not - Ignore

Adds the next word to the global ignored words list. Any word that substring matches the ignored word will be completely ignored by Prosetta. In the example below, the word was is ignored by the compiler, but wizards (which still triggers the was alias) is not. This causes the print statement to treat soup as a string (since it is not defined) but treat stew as a variable (since it is defined), leading to the output it gives.

## Arguments

```ignored (word)```

## Example

<editor :code='`
not was.
was soup two.
wizards stew three.
pri soup.
pri stew.
`' 
:code-wordier=null
output-method='console'></editor>