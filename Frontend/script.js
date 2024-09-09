var sourcecode, ctx, cnsl, canvas;
var has_run = false;

function init() {
  sourcecode = document.getElementById("sourcecode");
  canvas = document.getElementById("outputcanvas");
  ctx = canvas.getContext('2d');
  set_stroke("black");
  set_fill("transparent");
  ctx.lineWidth = 1;
  cnsl = document.getElementById("console");

  console.log(sourcecode, ctx, cnsl);
  print_console("Welcome to Prosetta!");
  clear_canvas();
}

function print_console() {
  let args = [];
  for (let i = 0; i < arguments.length; i++) {
    args.push(arguments[i]);
  }
  let line = args.join(" ")
  console.log(line);
  cnsl.innerText += line + "\n";
}

function draw_line(x1, y1, x2, y2) {
  ctx.beginPath();
  ctx.moveTo(x1, y1);
  ctx.lineTo(x2, y2);
  ctx.stroke();
}

function draw_rect(x1, y1, width, height) {
  ctx.beginPath();
  ctx.rect(x1 - width / 2, y1 - height / 2, width, height);
  ctx.stroke();
  ctx.fill();
}

function draw_ellipse(x1, y1, width, height) {
  ctx.beginPath();
  ctx.ellipse(x1, y1, width / 2, height / 2, 0, 0, 2 * Math.PI);
  ctx.stroke();
  ctx.fill();
}

function set_stroke(color) {
  ctx.strokeStyle = color;
}

function set_fill(color) {
  ctx.fillStyle = color;
}

function clear_canvas() {
  ctx.clearRect(0, 0, canvas.width, canvas.height);
}

function log_base(base, val) {
  return Math.log(val) / Math.log(base);
}

function log_base(val) {
  return Math.log(val) / Math.LN10;
}

function get_color(name) {
  return name;
}

function get_color(r, g, b) {
  return `rgb(${r}, ${g}, ${b})`
}

function runcode() {
  if (has_run) {
    print_console();
  }
  has_run = true;
  init();
  print_console("---");
  print_console();
  try {
    eval(sourcecode.value);
  } catch (error) {
    print_console(error);
  }
  cnsl.scrollTop = cnsl.scrollHeight;
}
