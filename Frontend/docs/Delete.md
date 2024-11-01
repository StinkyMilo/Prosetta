# Del - List Delete

Delete an item from a list. By default, this deletes from the beginning of the list, but an override allows deletion at any point within the list, shifting the values after it one space back.

## Arguments

### Start of List

```list (list)```

### Specified Location

```list (list), index (number)```

## Example
<editor :code='`
was list lis 1 2 3..
del list 1.
pri list.
`' 
:code-wordier="`
Was list listing 1, 2, and 3?
If so, delete the listed one.
Print the list.
`"
output-method='console'></editor>