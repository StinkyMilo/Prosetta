`Tangent Example`;
"By Milo Jacobs, The Trigonometry Baby";

let num_var = 0;
while_loop(() => (num_var < 360), () => {
  print_console(`Tangent of ${num_var} is`, Math.tan(num_var * Math.PI / 180));
  num_var = (num_var + 45);
});
