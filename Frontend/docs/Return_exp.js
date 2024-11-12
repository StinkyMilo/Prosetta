`Return Example`;
"By Milo Jacobs, Lisa Perfunctory";

function factorial_var(man_var) {
  if (man_var == 1) {
    return 1;
  }
  return (man_var * factorial_var((man_var - 1)));
}
let value_var = factorial_var(5);
print_console(value_var);
