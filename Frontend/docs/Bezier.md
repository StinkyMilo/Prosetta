# Bez - Bezier Curve

Draws a bezier curve with any number of control points followed by an endpoint. The bezier curve coordinates are required to be relative, meaning that the start point will always be the turtle location.

## Arguments

```pointX, pointY, … (numbers, must be an even number of args), endX (number), endY (number)```

## Example

<editor :code="`
fil red.
sto 0.
mov 0 -100.
bez 80 50 50 50 20 100 -70 0 -60 0 -20 -40.
bez -20 40 -60 0 -70 0 20 -100 50 -50 80 -50.
mov -50 60.
bez -30 0 0 -30.
fil 0.
pen 15.
sto 255 200 200.
mov 0 -90.
bez 80 50 15 50.
sto darkred.
mov 0 -100.
bez 80 50 50 50 20 100 -70 0 -60 0 -20 -40.
bez -20 40 -60 0 -70 0 20 -100 50 -50 80 -50.
pen 5.
sto black.
`" 
:code-wordier=null
output-method='canvas'></editor>