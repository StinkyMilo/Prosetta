# Rec - Rectangle

Draws a rectangle centered at the turtle's current coordinates. Turtle rotation will cause the rectangle to be rotated appropriately. Takes a specified width and height, and has overrides for absolute drawing locations. Has overrides for drawing a square and for drawing at an absolute position. Absolute overrides will call move to implicitly.

## Arguments

### Saure, Relative

```sideLength (number)```

### Square, Absolute

```x (number), y (number), sideLength (number)```

### Rectangle, Relative

```width (number), height (number)```

### Rectangle, Absolute

```x (number), y (number), width (number), height (number)```

## Example

<editor :code='`
rec 30 50.
`' 
:code-wordier="`
Draw a rectangle around my thirty fifty dollar bills!
`"
output-method='canvas'></editor>