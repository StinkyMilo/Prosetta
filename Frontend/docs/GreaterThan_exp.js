/*
Title: Greater Than Example
Primary author: Milo Jacobs
Imports: 
*/

let var_var = 20;
while_loop(() => (var_var > 10), () => {
  print_console(var_var);
  var_var = (var_var - 1);
  return _RETURN;
});
