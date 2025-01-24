# Lin - Line

Draws a line without ending the current shape. The relative-coordinate overrides do not start a new shape, but the absolute-coordinate overrides do (since they implicitly call move_to). Line can be called with one argument to move in the current turtle direction or with two to move right and forward by a specified amount from the current turtle position. Either of these versions can have two additional coordinates added to the beginning to make an implicit move_to call.

## Arguments

### Relative, Angled

```length (number)```

### Relative, X-Y

```xMove (number), yMove (number)```

### Absolute, Angled

```xStart (number), yStart (number), length (number)```

### Absolute, X-Y

```xStart (number), yStart (number), xMove (number), yMove (number)```

## Example

<editor :code="`
Line Example
by Milo Jacobs and John Graham\n
lin 10 10.
lin 50.
lin 30 30 40.
lin 20 20 30 30.
`" 
:code-wordier="`
Line Example
by Milo Jacobs and John Graham\n
Line up at ten past ten!
Line up with 50 people per row!
Line up thirty people in thirty rows and forty columns!
Make a line of 20 by 20, 30 by 30, five hundred by a thousand! 
I don't care, just do it!
`"
output-method='canvas'></editor>