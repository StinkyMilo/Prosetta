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
      postMessage({ command: "finished", data: { functions: functions, prosetta: data.prosetta } });
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

function draw_star() {
  queue_function("draw_star", arguments);
}

function draw_poly() {
  queue_function("draw_poly", arguments);
}

function draw_tri() {
  queue_function("draw_tri", arguments);
}

function draw_heart() {
  queue_function("draw_heart", arguments);
}

function draw_round_rec() {
  queue_function("draw_round_rec", arguments);
}

function draw_kirby() {
  queue_function("draw_kirby", arguments);
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

function get_random(...args) {
  let a = 0, b = 0;
  switch (args.length) {
    case 0:
      return Math.random();
    case 1:
      a = args[0];
      break;
    case 2:
      a = args[0];
      b = args[1];
      break;
  }
  let range = Math.abs(a - b);
  let offset = Math.min(a, b);
  return Math.floor(Math.random() * range) + offset;
}

function while_loop(predicate, callback) {
  let pred_aws = predicate();
  if (typeof pred_aws == "number") {
    for (let j = 0; j < pred_aws; j++) {
      callback();
    }
  } else {
    while (pred_aws) {
      callback();
      pred_aws = predicate();
    }
  }

}

function for_loop(list, callback) {
  if (typeof list == "number") {
    for (let j = 0; j < list; j++) {
      callback(j);
    }
  } else {
    for (x of list) {
      callback(x);
    }
  }
}

function end_shape() {
  queue_function("end_shape", arguments);
}

function queue_function(name, args) {
  functions.push({ name: name, args: [...args] });
}

