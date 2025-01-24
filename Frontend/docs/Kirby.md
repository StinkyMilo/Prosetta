# Kir - Kirby

Draws a photorealistic Kirby the dog, either at the turtle's position or at a specified relative position, of a specified size. It can be stretched along the x and y axes, but it is unaffected by stroke and fill color.

## Arguments

### Relative, Regular
```size (number)```

### Relative, Stretched
```width (number), height (number)```

### Absolute, Regular
```x (number), y (number), size (number)```

### Absolute, Stretched
```x (number), y (number), width (number), height (number)```

## Example

<editor :code="`
Kirby Example
by Milo Jacobs and Stampton G. Stampton\n
kir 400 400.
`"
:code-wordier="`
Kirby Example
by Milo Jacobs and Stampton G. Stampton\n
This brutal killer cannot get away. He killed 400 people over the course of 400 days!
If you see the face to your right, alert the police immediately. He is a dangerous criminal.
`"
output-method='canvas'>
</editor>