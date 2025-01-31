/*
Title: Sine Example
Primary author: Milo Jacobs
Imports: Trig
*/

let num_var = 0;
while_loop(() => (num_var < 360), () => {
  print_console(`Sine of ${num_var} is`, Math.sin(num_var * Math.PI / 180));
  num_var = (num_var + 45);
});
