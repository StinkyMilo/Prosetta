onmessage = async function(e) {
  let command = e.data.command;
  let data = e.data.data;
  switch (command) {
    case "run":
      eval(data.code)
      break;
  }
}

function print_console() {
  send_function("print_console", ...arguments);
}

function bezier_point() {
  send_function("bezier_point", ...arguments);
}

function draw_bezier() {
  send_function("draw_bezier", ...arguments);
}

function draw_line() {
  send_function("draw_line", ...arguments);
}

function move_to() {
  send_function("move_to", ...arguments);
}

function rotate_delta() {
  send_function("rotate_delta", ...arguments);
}

function reset_rotation() {
  send_function("reset_rotation", ...arguments);
}

function draw_rect() {
  send_function("draw_rect", ...arguments);
}

function draw_ellipse() {
  send_function("draw_ellipse", ...arguments);
}

function set_stroke() {
  send_function("set_stroke", ...arguments);
}

function set_fill() {
  send_function("set_fill", ...arguments);
}

function set_line_width() {
  send_function("set_line_width", ...arguments);
}

function get_concat_value() {
  send_function("get_concat_value", ...arguments);
}

function log_base() {
  send_function("log_base", ...arguments);
}

function get_color() {
  send_function("get_color", ...arguments);
}

function send_function(name, args) {
  postMessage({ command: "function", data: { name: name, args: args } });
}
