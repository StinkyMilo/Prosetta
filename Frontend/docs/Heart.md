# Hea - Heart

Draws a heart, either at the turtle's position or at a specified relative position, of a specified size. It can be stretched along the x and y axes.

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
Heart Example
by Milo Jacobs, John Graphics, and Stampton G. Stampton\n
hea 100.
pen 3.
sto red.
fil pink.
`"
:code-wordier="`
Heart Example
by Milo Jacobs, John Graphics, and Stampton G. Stampton\n
This Health Pack restores 100 HP.
Press E to open your inventory and use it!
If you have any questions, just talk to me! 
A tip: This red Health Pack restores 100 HP, but on higher difficulties you'll find pink Health Packs, which heals only half the health.
`"
output-method='canvas'>
</editor>