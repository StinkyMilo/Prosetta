# The Shape Drawing System

The drawing system is a cross between standard JavaScript canvases and turtle graphics, with overrides for most functions allowing both options. Your "turtle" has a position and a rotation and can be moved throughout the 400x400 canvas. Whenever you call a drawing function, it is assumed that your drawing will start from the position of your turtle and with the rotation of your turtle. However, overrides are also provided to move your turtle's position and draw something in one function call.

In addition to basic shapes like ellipses and rectangles, you can draw arbitrary polygons out of lines. When you change the stroke or fill color, Prosetta will also stroke and fill the currently active shape. If the program ends without changing the stroke or fill color, or if you start a new shape through a variety of methods, the last-used settings will be used to end the shape. This should make it so that you never have to end a shape manually.
