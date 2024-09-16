var sourcecode, ctx, cnsl, canvas;
var x = 0, y = 0, rotation = 0;
var has_run = false;
var has_drawn_shape = false;
var last_was_line = false;

function init() {
  sourcecode = document.getElementById("sourcecode");
  canvas = document.getElementById("outputcanvas");
  ctx = canvas.getContext('2d');
  cnsl = document.getElementById("console");

  print_console("Welcome to Prosetta!");
  reset_rotation();
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
}

function draw_line() {
  switch (arguments.length) {
    case 1:
      if (!last_was_line) {
        start_shape();
        ctx.moveTo(x, y);
      }
      move_distance(arguments[0]);
      console.log(x, y, arguments[0], rotation);
      ctx.lineTo(x, y);
      break;
    case 2:
      if (!last_was_line) {
        start_shape();
        ctx.moveTo(x, y);
      }
      move_delta(arguments[0], arguments[1]);
      ctx.lineTo(x, y);
      break;
    case 3:
      start_shape();
      move_to(arguments[0], arguments[1]);
      ctx.moveTo(x, y);
      move_distance(arguments[2]);
      ctx.lineTo(x, y);
      break;
    case 4:
      start_shape();
      move_to(arguments[0], arguments[1]);
      ctx.moveTo(x, y);
      move_to(arguments[2], arguments[3]);
      ctx.lineTo(x, y);
      break;

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

function move_distance(dist) {
  let dx = Math.cos(rotation_radians()) * dist;
  let dy = Math.sin(rotation_radians()) * dist;
  move_delta(dx, dy);
}

function rotation_radians() {
  return -rotation * Math.PI / 180;
}

function rotate_delta(deg) {
  rotation += deg;
}

function reset_rotation() {
  rotation = 0;
}

function rotate_point(cx, cy, rad, x1, y1) {
  let cos = Math.cos(rad),
    sin = Math.sin(rad),
    nx = (cos * (x1 - cx)) + (sin * (y1 - cy)) + cx,
    ny = (cos * (y1 - cy)) - (sin * (x1 - cx)) + cy;
  return [nx, ny];
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
  // let x1, y1, x2, y2, x3, y3, x4, y4;
  let rad = rotation_radians()
  let [x1, y1] = rotate_point(x, y, rad, x - width / 2, y - height / 2);
  let [x2, y2] = rotate_point(x, y, rad, x + width / 2, y - height / 2);
  let [x3, y3] = rotate_point(x, y, rad, x + width / 2, y + height / 2);
  let [x4, y4] = rotate_point(x, y, rad, x - width / 2, y + height / 2);
  ctx.moveTo(x1, y1);
  ctx.lineTo(x2, y2);
  ctx.lineTo(x3, y3);
  ctx.lineTo(x4, y4);
  ctx.closePath();
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
  ctx.ellipse(x, y, width / 2, height / 2, -rotation_radians(), 0, 2 * Math.PI);
  ctx.closePath();
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
