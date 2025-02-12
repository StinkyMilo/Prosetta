/*
Title: Sine Example
Primary author: Milo Jacobs
Imports: Trig
*/

let var_num = 0;
while_loop(() => (var_num < 360), () => {
  print_console(`Sine of ${var_num} is`, Math.sin(var_num * Math.PI / 180));
  var_num = (var_num + 45);
  return _RETURN;
});
