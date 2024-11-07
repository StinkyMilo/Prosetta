# Mov - Move To

Moves the cursor to a new position. This is implicitly called in any "absolute" shape function. It ends any currently active shape, sets the turtle position to the specified value, and begins a new shape at that location.
	
    
Alternatively, if given a single argument, the turtle will move the specified distance in the direction it is facing without starting a new shape.

## Arguments

### Absolute

x (number), y (number)

### Relative

length (number)

## Example

<editor :code="`
Move To Example
by Milo Jacobs and John Graphics\n
arc 50.
mov 100 100.
arc 50.
`" 
:code-wordier="`
Move To Example
by Milo Jacobs and John Graphics\n
March 50 feet!
Move 100 meters and 100 meters again!
March 50 more feet!
`"
output-method='canvas'></editor>