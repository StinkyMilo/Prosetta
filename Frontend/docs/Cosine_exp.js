`Cosine Example`;
"By Milo Jacobs, The Trigonometry Baby";

let num_var = 0;
while ((num_var < 360)) {
  print_console(`Cosine of ${num_var} is`, Math.cos(num_var*Math.PI/180));
  num_var = (num_var + 45);
}
