`My Poem`;
"By Milo Jacobs, John Graphton, Alice Framingham";

let frame_var = (_frame / 5);
while ((frame_var > 20)) {
  frame_var = (frame_var - 20);
}
rotate_delta(((frame_var * frame_var) / 2));
draw_rect((frame_var * frame_var));
