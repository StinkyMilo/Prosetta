# App - List Append

Add an item to a list. By default, this appends to the end of the list, but an override allows it to be inserted at any location in the list, shifting the values after it one space forward.

## Arguments

### End of List

```list (list), value (any)```

### Specified Location

```list (list), value (any), index (number)```

## Example
<editor :code='`
was list lis 1 2 3..
app list four.
pri list.
`' 
:code-wordier="`
Was list listing 1, 2, and 3?
Append a list four times.
Then print the list.
`"
output-method='console'></editor>