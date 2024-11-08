# Fra - Frame
To enable Prosetta's animation system, you must import the Frame library with the substring `fram` (see [Imports](Imports.md) for more information on how to import a library). Any program with the library imported works slightly differently than one without. 

Any program with animation enabled will, rather than being run once when the document changes, be run repeatedly on a loop, clearing the canvas after each run. Enabling animation also gives you access to the current number of _frames_ since the program started runnning, accessible by the `fra` alias. 

Any word that triggers `fra` is treated like a variable, which means that, unlike other aliases, _it needs no closing punctuation_. The value will change each time the code is run, averaging 20-25 frames per second on most computers.

## Example
<editor :code='`
Frame Example
by Milo Jacobs, Lord Framingham III, and John Graphton.\n
arc 0 sub mod tim 4 fra. 400. 200. 30.
fil blue.
`' 
:code-wordier="`
Frame Example
by Milo Jacobs, Lord Framingham III, and John Graphton.\n
March, you litany.
    Subtract a modicum of time from four to frame. 
    Walk four-hundred yards, then two-hundred, then thirty.
fil blue.
`"
output-method='canvas'></editor>

