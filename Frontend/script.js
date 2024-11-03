import { wordsForAliases } from './wordsForAliases.js';

var jscode, sourcecode, ctx, cnsl, canvas;
var x = 0, y = 0, rotation = 0;
var has_run = false;
var has_drawn_shape = false;
var last_shape = "none";
var worker;
var editor;
let tooltips = [];

function init_canvas() {
  sourcecode = document.getElementById("code");
  jscode = document.getElementById("js");
  canvas = document.getElementById("outputcanvas");
  ctx = canvas.getContext('2d');
  cnsl = document.getElementById("console");

  canvas.width = canvas.width;
  has_drawn_shape = false;
  last_shape = "none";
  reset_rotation();
  _move_to(0, 0);
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
  if (last_shape == "none" || last_shape == "move") {
    return;
  }
  ctx.fill();
  ctx.stroke();
  last_shape = "none";
}

function start_shape() {
  end_shape();
  ctx.beginPath();
}

function lerp(a, b, t) {
  return (1 - t) * a + t * b;
}

function bezier_point(t, points) {
  if (points.length == 1) {
    return points[0];
  }

  let new_points = [];
  for (let i = 0; i < points.length - 1; i++) {
    let x = lerp(points[i].x, points[i + 1].x, t);
    let y = lerp(points[i].y, points[i + 1].y, t);
    new_points.push({ x: x, y: y });
  }
  return bezier_point(t, new_points);
}

function draw_bezier(...xy) {
  if (last_shape != "line") {
    start_shape();
    ctx.moveTo(x, y);
  }
  let points = [{ x: x, y: y }];
  for (let i = 0; i < xy.length; i += 2) {
    move_delta(xy[i], xy[i + 1]);
    points.push({ x: x, y: y });
  }
  for (let t = 0; t < 1; t += 0.05) {
    let point = bezier_point(t, points)
    ctx.lineTo(point.x, point.y);
  }
  let point = bezier_point(1, points)
  ctx.lineTo(point.x, point.y);
  last_shape = "line";
}

function draw_line() {
  switch (arguments.length) {
    case 1:
      if (last_shape != "line") {
        start_shape();
        ctx.moveTo(x, y);
      }
      move_distance(arguments[0]);
      console.log(x, y, arguments[0], rotation);
      ctx.lineTo(x, y);
      break;
    case 2:
      if (last_shape != "line") {
        start_shape();
        ctx.moveTo(x, y);
      }
      move_delta(arguments[0], arguments[1]);
      ctx.lineTo(x, y);
      break;
    case 3:
      start_shape();
      _move_to(arguments[0], arguments[1]);
      ctx.moveTo(x, y);
      move_distance(arguments[2]);
      ctx.lineTo(x, y);
      break;
    case 4:
      start_shape();
      _move_to(arguments[0], arguments[1]);
      ctx.moveTo(x, y);
      _move_to(arguments[2], arguments[3]);
      ctx.lineTo(x, y);
      break;

  }
  last_shape = "line";
}

function move_to() {
  end_shape();
  last_shape = "move";
  if (arguments.length == 1) {
    move_distance(arguments[0]);
  }
  else {
    _move_to(arguments[0], arguments[1]);
  }
}

function _move_to(x1, y1) {
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
      _move_to(arguments[0], arguments[1]);
      width = arguments[2];
      height = arguments[2];
      break;
    case 4:
      _move_to(arguments[0], arguments[1]);
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
  last_shape = "rect";
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
      _move_to(arguments[0], arguments[1]);
      width = arguments[2];
      height = arguments[2];
      break;
    case 4:
      _move_to(arguments[0], arguments[1]);
      width = arguments[2];
      height = arguments[3];
      break;
  }
  start_shape();
  ctx.ellipse(x, y, width / 2, height / 2, -rotation_radians(), 0, 2 * Math.PI);
  ctx.closePath();
  last_shape = "ellipse";
}

function set_stroke(...color) {
  ctx.strokeStyle = conv_color(...color);
}

function set_fill(...color) {
  ctx.fillStyle = conv_color(...color);
}

function conv_color(...color) {
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

function set_line_width(width) {
  ctx.lineWidth = width;
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

function get_color(...args) {
  if (args.length == 1) {
    return args[0];
  }
  return `rgb(${args[0]}, ${args[1]}, ${args[2]})`;
}

function runCode() {
  if (has_run) {
    print_console();
    print_console("Welcome to Prosetta!");
    print_console("---");
    print_console();
  }
  has_run = true;
  init_canvas();
  try {
    eval(jscode.value);
    end_shape();
  } catch (error) {
    print_console(error);
  }
  cnsl.scrollTop = cnsl.scrollHeight;
}

function openTab(event, tab) {
  let tabContents = document.getElementsByClassName("tabContent");
  for (let i = 0; i < tabContents.length; i++) {
    if (tabContents[i].id == tab) {
      tabContents[i].style.display = "block";
    }
    else {
      tabContents[i].style.display = "none";
    }
  }
  let tabs = document.getElementsByClassName("tabBtn");
  for (let i = 0; i < tabs.length; i++) {
    tabs[i].className = tabs[i].className.replace(" active", "");
  }
  event.currentTarget.className += " active";
}

function updateCode() {
  if (editor == null) {
    return;
  }
  msg_worker("changed", editor.getValue());
}

async function initialize(startingCode) {
  let tabs = document.getElementsByClassName("tabBtn tabDefault");
  tabs[0].click();

  setup_webworker();
  worker.postMessage({ command: "initialize" });
  init_canvas();
  print_console("Welcome to Prosetta!");
  print_console("---");
  print_console();
  updateCode();
  return setup_editor(startingCode);
}

window.runCode = runCode;
window.updateCode = updateCode;
window.openTab = openTab;

function msg_worker(command, data) {
  worker.postMessage({ command: command, data: data });
}

function setup_editor(startingCode) {
  editor = CodeMirror(document.getElementById("code"), {
    value: "",
    mode: "plaintext"
  });
  editor.setSize("100%", "100%");

  /*
    Returns a node that contains the alternate word suggestions
  */
  function getNewTooltip(alias) {
    //For now, don't use rust endpoints; just choose the first alias.
    //Later, we'll want to use the rust endpoints though
    let widget = document.createElement("div");
    widget.className = "tooltip";
    let header = document.createElement("h1");
    let u = document.createElement("u");
    header.appendChild(u);
    u.innerHTML = alias;
    widget.appendChild(header);
    let words = wordsForAliases[alias];
    for (let i = 0; i < words.length; i++) {
      let wordElement = document.createElement("p");
      wordElement.innerHTML = words[i];
      widget.appendChild(wordElement);
      wordElement.onclick = ()=>{
        editor.replaceRange(words[i],currentWordStart,currentWordEnd);
        currentWordEnd = {line: currentWordStart.line, ch: currentWordStart.ch + words[i].length};
      };
    }
    return widget;
  }

  let activeWidget;
  let lastWordPos = { line: -1, ch: -1 };
  let displayTimeout;
  let removeTimeout;
  let currentWordStart;
  let currentWordEnd;

  function clearWidget() {
    // removeWithFadeout(activeWidget);
    // console.log("REMOVING");
    activeWidget?.remove();
    activeWidget = null;
    lastWordPos = { line: -1, ch: -1 };
    if (displayTimeout != null) {
      clearTimeout(displayTimeout);
      displayTimeout = null;
    }
  }

  //This isn't working. TODO fix
  function removeWithFadeout(element) {
    if (element == null) {
      return;
    }
    element.style.animation = "";
    element.style.transition = "opacity 0.5s ease";
    element.style.opacity = 1;
    console.log(element.style);
    setTimeout(() => {
      element.remove();
    }, 500);
  }

  window.onmousemove = function(e) {
    if (activeWidget != null && (e.target == activeWidget || activeWidget.contains(e.target))) {
      if (removeTimeout != null) {
        clearTimeout(removeTimeout);
        removeTimeout = null;
      }
      return;
    }
    let pos = { left: e.clientX, top: e.clientY + window.scrollY };
    let textPos = editor.coordsChar(pos);
    // console.log(pos,textPos);
    // console.log(editor.charCoords({ch:0,line:0}),pos);
    if (textPos.outside) {
      if (removeTimeout == null && activeWidget != null) {
        removeTimeout = setTimeout(() => {
          clearWidget();
        }, 250);
      }
      return;
    }
    let wordPos = editor.findWordAt(textPos);
    let word = editor.getRange(wordPos.anchor, wordPos.head).toLowerCase();
    let midPos = { line: 0, ch: 0 };
    if (wordPos.head.line == wordPos.anchor.line) {
      midPos = { line: wordPos.head.line, ch: (wordPos.head.ch + wordPos.anchor.ch) / 2 };
    } else {
      midPos = wordPos.head;
    }
    if (word.match(/^\s*$/) || (midPos.line == lastWordPos.line && midPos.ch == lastWordPos.ch)) {
      return;
    }
    if (removeTimeout != null) {
      clearTimeout(removeTimeout);
      removeTimeout = null;
    }
    if (displayTimeout != null) {
      clearTimeout(displayTimeout);
      displayTimeout = null;
    }
    let alias = null;
    let txtInd = editor.indexFromPos(textPos);
    for(let i = 0; i < tooltips.length; i++){
      if(tooltips[i].start <= txtInd && txtInd <= tooltips[i].end){
        alias = tooltips[i].alias;
        break;
      }
    }
    if (alias == null) {
      return;
    }
    console.log(alias);
    displayTimeout = setTimeout(() => {
      clearWidget();
      currentWordStart = wordPos.anchor;
      currentWordEnd = wordPos.head;
      activeWidget = getNewTooltip(alias);
      lastWordPos = midPos;
      editor.addWidget(midPos, activeWidget);
    }, 500);
  }

  editor.on("change", (cm, change) => { updateCode(); });
  editor.setValue(startingCode);
  return editor;
  /**
   * cursorActivity event gets when cursor or selection moves
   * beforeCursorEnter event fires when the cursor enters the marked range
   * doc.replaceSelection will replace the current selection with a given string
   * doc.getCursor retrieves one end of the primary selection
   * cm.findWordAt returns the start and end of the word at a given position
   * doc.setBookmark might be what you want for a popup? I'm not 100% sure from the description. The widget would make sense
   * cm.addWidget might also be what you want. addLineWidget moves below lines down
   * show-hint extension shows autocomplete hints, not what we want now but could be useful later
   */

  /*
    Plan:
      Change cursorActivity to mouse move
      Find word start-end for mouse move
      If it's over a new word (not the word it was over last):
        Cancel any existing timeouts
        Start a new timeout
      If a timeout completes,
        Create a tooltip for the corresponding word, put at the word's end position
        
  
  */
}

function setup_webworker() {
  worker = new Worker("language_worker.js");
  worker.onmessage = e => {
    let command = e.data.command;
    let data = e.data.data;
    switch (command) {
      case "parsed":
        jscode.innerHTML = data.js;
        let highlights = data.hl;
        editor.doc.getAllMarks().forEach(marker => marker.clear());
        for (let hl of highlights) {
          editor.markText(
            { line: hl.line, ch: hl.index },
            { line: hl.line, ch: hl.index + hl.length },
            { className: hl.color.at(-1) }
          );
        }
        console.log(data.wordTriggers);
        tooltips = JSON.parse(data.wordTriggers);
        break;
    }
  };
}

export default initialize;
