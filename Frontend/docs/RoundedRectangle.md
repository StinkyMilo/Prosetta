# Hea - Heart

Draws a rounded rectangle, either at the turtle's position or at a specified relative position, of a specified size. It can be stretched along the x and y axes. The final argument is the border radius in pixels.

## Arguments

### Relative, Square
```size (number)```

### Relative, Stretched
```width (number), height (number)```

### Absolute, Square
```x (number), y (number), size (number)```

### Absolute, Stretched
```x (number), y (number), width (number), height (number)```

## Example

<editor :code="`
Rounded Rectangle Example
by Milo Jacobs, John Graphics, and Stampton G. Stampton\n
roc 90 40 10.
fil light grey.
`"
:code-wordier="`
Rounded Rectangle Example
by Milo Jacobs, John Graphics, and Stampton G. Stampton\n
This is a very big rock. It is 90 by 40 by 10 meters.
Like every rock, it is filled with millions of tiny light grey spiders that give it its color.
`"
output-method='canvas'>
</editor>