/*
Title: Bezier Example
Primary author: Milo Jacobs
Imports: Graph
*/

set_fill("red");
set_stroke(0);
move_to(0, -100);
draw_bezier(80, 50, 50, 50, 20, 100, -70, 0, -60, 0, -20, -40);
draw_bezier(-20, 40, -60, 0, -70, 0, 20, -100, 50, -50, 80, -50);
move_to(-50, 60);
draw_bezier(-30, 0, 0, -30);
set_fill(0);
set_line_width(15);
set_stroke(255, 200, 200);
move_to(0, -90);
draw_bezier(80, 50, 15, 50);
set_stroke("darkred");
move_to(0, -100);
draw_bezier(80, 50, 50, 50, 20, 100, -70, 0, -60, 0, -20, -40);
draw_bezier(-20, 40, -60, 0, -70, 0, 20, -100, 50, -50, 80, -50);
set_line_width(5);
set_stroke("black");
