# Arc - Ellipse

Draws an ellipse centered at the turtle's position with a width and a height. The "absolute coordinates" override will move the turtle to the specified location before drawing. The rotation of the ellipse will be determined by the turtle's rotation. The width and height are the height of the maximum bounds (diameter in the case of a circle).

## Arguments

### Circle, Relative

```diameter (number)```

### Circle, Absolute

```x (number), y (number), diameter (number)```

### Ellipse, Relative

```width (number), height (number)```

### Ellipse, Absolute

```x (number), y (number), width (number), height (number)```

## Example

<editor :code='`
Ellipse Example
by Milo Jacobs and Sarah Regraph\n
mov 30 30.
arc 50.
arc -30 -30 50 70.
`' 
:code-wordier="`
Ellipse Example
by Milo Jacobs and Sarah Regraph\n
Move thirty to thirty!
The arch is fifty years old!
March sub thirty. Then sub thirty. Fifty then seventy!
`"
output-method='canvas'></editor>