var functions = [];
onmessage = async function(e) {
  let command = e.data.command;
  let data = e.data.data;
  switch (command) {
    case "run":
      functions = [];
      try {
        var _frame = data.frame;
        eval(data.code);
        end_shape();
      } catch (error) {
        print_console(error);
      }
      postMessage({ command: "finished", data: functions });
      break;
  }
}

function print_console() {
  queue_function("print_console", arguments);
}

function bezier_point() {
  queue_function("bezier_point", arguments);
}

function draw_bezier() {
  queue_function("draw_bezier", arguments);
}

function draw_line() {
  queue_function("draw_line", arguments);
}

function move_to() {
  queue_function("move_to", arguments);
}

function rotate_delta() {
  queue_function("rotate_delta", arguments);
}

function reset_rotation() {
  queue_function("reset_rotation", arguments);
}

function draw_rect() {
  queue_function("draw_rect", arguments);
}

function draw_ellipse() {
  queue_function("draw_ellipse", arguments);
}

function set_stroke() {
  queue_function("set_stroke", arguments);
}

function set_fill() {
  queue_function("set_fill", arguments);
}

function set_line_width() {
  queue_function("set_line_width", arguments);
}

function get_concat_value(...args) {
  let total = 0;
  let multiplier = 1;
  for (let i = args.length - 1; i >= 0; i--) {
    total += args[i] * multiplier;
    multiplier *= 10;
  }
  return total;
}

function log_base(base, val = undefined) {
  if (val == undefined) {
    return Math.log(base);
  }
  return Math.log(val) / Math.log(base);
}

function get_color(...color) {
  switch (color.length) {
    case 1:
      if (color[0] == 0) {
        return "transparent";
      }
      return color[0];
    case 3:
      return `rgb(${color[0]}, ${color[1]}, ${color[2]})`;
  }
}

function end_shape() {
  queue_function("end_shape", arguments);
}

function queue_function(name, args) {
  functions.push({ name: name, args: [...args] });
}
