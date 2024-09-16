var sourcecode, ctx, cnsl, canvas;
var x, y;
var has_run = false;
var has_drawn_shape = false;
var last_was_line = false;

function init() {
  sourcecode = document.getElementById("sourcecode");
  canvas = document.getElementById("outputcanvas");
  ctx = canvas.getContext('2d');
  cnsl = document.getElementById("console");

  print_console("Welcome to Prosetta!");
  clear_canvas();
  has_drawn_shape = false;
  move_to(0, 0);
  ctx.moveTo(x, y);
  set_stroke("black");
  set_fill("transparent");
  ctx.lineCap = "round";
  ctx.lineJoin = "round";
  ctx.lineWidth = 1;
}

function print_console() {
  let args = [];
  for (let i = 0; i < arguments.length; i++) {
    args.push(arguments[i]);
  }
  let line = args.join(" ")
  cnsl.innerText += line + "\n";
}

function end_shape() {
  if (!has_drawn_shape) {
    return;
  }
  ctx.fill();
  ctx.stroke();
}

function start_shape() {
  end_shape();
  ctx.beginPath();
  ctx.moveTo(x, y);
}

function draw_line() {
  if (arguments.length == 2) {
    if (!last_was_line) {
      start_shape();
    }
    move_delta(arguments[0], arguments[1]);
    ctx.lineTo(x, y);
  }
  else {
    start_shape();
    move_to(arguments[0], arguments[1]);
    ctx.moveTo(x, y);
    move_to(arguments[2], arguments[3]);
    ctx.lineTo(x, y);
  }
  last_was_line = true;
  has_drawn_shape = true;
}

function move_to(x1, y1) {
  x = x1 + 200;
  y = 200 - y1;
}

function move_delta(x1, y1) {
  x += x1;
  y -= y1;
}

function draw_rect() {
  let width, height;
  switch (arguments.length) {
    case 1:
      width = arguments[0];
      height = arguments[0];
      break;
    case 2:
      width = arguments[0];
      height = arguments[1];
      break;
    case 3:
      move_to(arguments[0], arguments[1]);
      width = arguments[2];
      height = arguments[2];
      break;
    case 4:
      move_to(arguments[0], arguments[1]);
      width = arguments[2];
      height = arguments[3];
      break;
  }
  start_shape();
  ctx.rect(x - width / 2, y - height / 2, width, height);
  last_was_line = false;
  has_drawn_shape = true;
}

function draw_ellipse() {
  let width, height;
  switch (arguments.length) {
    case 1:
      width = arguments[0];
      height = arguments[0];
      break;
    case 2:
      width = arguments[0];
      height = arguments[1];
      break;
    case 3:
      move_to(arguments[0], arguments[1]);
      width = arguments[2];
      height = arguments[2];
      break;
    case 4:
      move_to(arguments[0], arguments[1]);
      width = arguments[2];
      height = arguments[3];
      break;
  }
  start_shape();
  ctx.ellipse(x, y, width / 2, height / 2, 0, 0, 2 * Math.PI);
  last_was_line = false;
  has_drawn_shape = true;
}

function set_stroke(color) {
  if (color == 0) {
    ctx.strokeStyle = "transparent";
  } else {
    ctx.strokeStyle = color;
  }
}

function set_fill(color) {
  if (color == 0) {
    ctx.strokeStyle = "transparent";
  } else {
    ctx.fillStyle = color;
  }
}

function set_line_width(width) {
  ctx.lineWidth = width;
}

function clear_canvas() {
  set_fill("white");
  set_stroke("transparent");
  draw_rect(0, 0, 400, 400);
  end_shape();
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
    end_shape();
  } catch (error) {
    print_console(error);
  }
  cnsl.scrollTop = cnsl.scrollHeight;
}
