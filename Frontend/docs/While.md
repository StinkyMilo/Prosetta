# Whi - While

A while loop. If the condition (first argument) is true, all other arguments (statements) will be executed. Then the process is repeated, all statements executing until the condition is no longer true.

If the boolean condition is instead a number, the statements will all be activated a number of times equal to that number. 

## Arguments

```condition (any), [st_1, ... st_∞] (statements)```

## Special Case

```repeat (number), [st_1, ... st_∞] (statements)```

## Example
<editor :code='`
While Example
by Milo Jacobs\n
was var one.
whi les var 10:
pri var.
was var add var one...\n
whi 10.
pri 10.
.
`' 
:code-wordier="`
While Example
by Milo Jacobs\n
Was var one?
While it was less, that var, than 10:
I print the var.
After then was var add var to one...\n
While I'm still 10 years old,
I'll practice soccer for 10 hours a day!
`"
output-method='console'></editor>