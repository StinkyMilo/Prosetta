# Fil - Fill Shape

Sets the fill color to a specified value and ends the current shape with the new fill value. The color can be made with the color constructor or be a literal color name. You can also enter RGB values directly, or a single greyscale value. If the single greyscale value is 0, the color will be transparent.

## Arguments

### Color
```color (color)```

### RGB
```red (number), green (number), blue (number)```

### One Number
```greyscale (color)```

## Example

<editor :code="`
Fill Example
by Milo Jacobs and John Graphton\n
arc 50.
fil red.
`" 
:code-wordier="`
Fill Example
by Milo Jacobs and John Graphton\n
You must march fifty meters.
Filthy red balloon for you.
`"
output-method='canvas'></editor>