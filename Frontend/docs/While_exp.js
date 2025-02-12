/*
Title: While Example
Primary author: Milo Jacobs
Imports: 
*/

let var_var = 1;
while_loop(() => (var_var < 10), () => {
  print_console(var_var);
  var_var = (var_var + 1);
  return _RETURN;
});
while_loop(() => 10, () => {
  print_console(10);
  return _RETURN;
});
