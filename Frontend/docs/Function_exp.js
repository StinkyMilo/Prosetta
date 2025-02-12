/*
Title: Function Example
Primary author: Milo Jacobs
Imports: List, Func
*/

function var_factorial(var_man) {
  if (var_man == 1) {
    return 1;
  }
  return (var_man * var_factorial((var_man - 1)));
}
let var_value = var_factorial(5);
print_console(var_value);
