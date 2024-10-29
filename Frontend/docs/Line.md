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
lin 10 10.
`" 
:code-wordier="`
Line up at ten past ten!
`"
output-method='canvas'></editor>