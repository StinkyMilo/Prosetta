import { allWords, wordsForAliases } from './wordsForAliases.js';
import { Import } from './wasm-bindings/prosetta.js';

var jscode, sourcecode, cnsl, curr_ctx, curr_canvas, displayed_ctx, displayed_canvas;
var x = 0, y = 0, rotation = 0;
var has_drawn_shape = false;
var last_shape = "none";
var language_worker, runner_worker;
var editor;
let tooltips = [];
var imports = [];
var frameInterval;
var curr_frame = 0;
var latest_frame = 0;
var last_frame_timestamp = Date.now();
var target_fps = 30;

function init_canvas() {
  cnsl.innerText = "";
  curr_canvas.width = curr_canvas.width;
  has_drawn_shape = false;
  last_shape = "none";
  reset_rotation();
  _move_to(0, 0);
  curr_ctx.moveTo(x, y);
  set_stroke("black");
  set_fill("transparent");
  curr_ctx.lineCap = "round";
  curr_ctx.lineJoin = "round";
  curr_ctx.lineWidth = 1;
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
  curr_ctx.fill();
  curr_ctx.stroke();
  last_shape = "none";
}

function start_shape() {
  end_shape();
  curr_ctx.beginPath();
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
    curr_ctx.moveTo(x, y);
  }
  let points = [{ x: x, y: y }];
  for (let i = 0; i < xy.length; i += 2) {
    move_delta(xy[i], xy[i + 1]);
    points.push({ x: x, y: y });
  }
  for (let t = 0; t < 1; t += 0.05) {
    let point = bezier_point(t, points)
    curr_ctx.lineTo(point.x, point.y);
  }
  let point = bezier_point(1, points)
  curr_ctx.lineTo(point.x, point.y);
  last_shape = "line";
}

function draw_line() {
  switch (arguments.length) {
    case 1:
      if (last_shape != "line") {
        start_shape();
        curr_ctx.moveTo(x, y);
      }
      move_distance(arguments[0]);
      curr_ctx.lineTo(x, y);
      break;
    case 2:
      if (last_shape != "line") {
        start_shape();
        curr_ctx.moveTo(x, y);
      }
      move_delta(arguments[0], arguments[1]);
      curr_ctx.lineTo(x, y);
      break;
    case 3:
      start_shape();
      _move_to(arguments[0], arguments[1]);
      curr_ctx.moveTo(x, y);
      move_distance(arguments[2]);
      curr_ctx.lineTo(x, y);
      break;
    case 4:
      start_shape();
      _move_to(arguments[0], arguments[1]);
      curr_ctx.moveTo(x, y);
      _move_to(arguments[2], arguments[3]);
      curr_ctx.lineTo(x, y);
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
  curr_ctx.moveTo(x1, y1);
  curr_ctx.lineTo(x2, y2);
  curr_ctx.lineTo(x3, y3);
  curr_ctx.lineTo(x4, y4);
  curr_ctx.closePath();
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
  curr_ctx.ellipse(x, y, width / 2, height / 2, -rotation_radians(), 0, 2 * Math.PI);
  curr_ctx.closePath();
  last_shape = "ellipse";
}

function set_stroke(...color) {
  curr_ctx.strokeStyle = get_color(...color);
}

function set_fill(...color) {
  curr_ctx.fillStyle = get_color(...color);
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

function set_line_width(width) {
  curr_ctx.lineWidth = width;
}


function runCode() {
  init_canvas();
  print_console();
  print_console("Welcome to Prosetta!");
  print_console("---");
  print_console();
  runner_worker.postMessage({ command: "run", data: { code: jscode.innerText, frame: curr_frame } });
  // cnsl.scrollTop = cnsl.scrollHeight;
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
  last_frame_timestamp = Date.now();
  msg_worker("changed", editor.getValue());
}

async function initialize(startingCode) {
  language_worker?.terminate();
  sourcecode = document.getElementById("code");
  jscode = document.getElementById("js");
  curr_canvas = document.getElementById("outputcanvas");
  displayed_canvas = document.getElementById("outputcanvas2");
  curr_ctx = curr_canvas.getContext('2d');
  displayed_ctx = displayed_canvas.getContext('2d');
  jscode.innerText = "";
  cnsl = document.getElementById("console");
  let tabs = document.getElementsByClassName("tabBtn tabDefault");
  tabs[0].click();

  setup_webworker();
  language_worker.postMessage({ command: "initialize" });
  init_canvas();
  print_console("Welcome to Prosetta!");
  print_console("---");
  print_console();
  let editor = setup_editor(startingCode);
  return editor;
}


function msg_worker(command, data) {
  language_worker.postMessage({ command: command, data: data });
}

function getWordsOfLength(len){
  return allWords.filter((word)=>{
    return word.length == len;
  });
}

function getWordsThatContain(substr){
  return allWords.filter((word)=>{
    return word.indexOf(substr.toLowerCase()) > -1;
  });
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
  function getNewTooltip(tooltip) {
    //For now, don't use rust endpoints; just choose the first alias.
    //Later, we'll want to use the rust endpoints though
    let widget = document.createElement("div");
    widget.className = "tooltip";
    let header = document.createElement("h1");
    let u = document.createElement("u");
    let words;
    if(tooltip.type == "alias"){
      words = wordsForAliases[tooltip.value];
      u.innerHTML = "Words that trigger " + tooltip.value;
    }else if(tooltip.type == "length"){
      words = getWordsOfLength(tooltip.len);
      u.innerHTML = "Words of length " + tooltip.len;
    }else if(tooltip.type == "variable"){
      words = getWordsThatContain(tooltip.name);
      u.innerHTML = "Words that contain the variable " + tooltip.name;
    }
    header.appendChild(u);
    widget.appendChild(header);
    for (let i = 0; i < words.length; i++) {
      let wordElement = document.createElement("p");
      wordElement.innerHTML = words[i];
      widget.appendChild(wordElement);
      wordElement.onclick = () => {
        editor.replaceRange(words[i], currentWordStart, currentWordEnd);
        currentWordEnd = { line: currentWordStart.line, ch: currentWordStart.ch + words[i].length };
      };
    }
    return widget;
  }

  let activeWidget;
  let lastWordPos = { line: -1, ch: -1 };
  let displayTimeout;
  let removeTimeout;
  let currentWordStart = { line: -1, ch: -1 };
  let currentWordEnd = { line: -1, ch: -1 };
  let nextWordStart = {line: -1, ch: -1};
  let nextWordEnd = {line: -1, ch: -1};

  function clearWidget() {
    // removeWithFadeout(activeWidget);
    // console.log("REMOVING");
    activeWidget?.remove();
    activeWidget = null;
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
    lastWordPos = { line: -1, ch: -1 };
    console.log(element.style);
    setTimeout(() => {
      element.remove();
    }, 500);
  }

  window.onmousemove = function(e) {
    let pos = { left: e.clientX, top: e.clientY + window.scrollY };
    let textPos = editor.coordsChar(pos);
    // console.log(pos,textPos);
    // console.log(editor.charCoords({ch:0,line:0}),pos);
    let wordPos = editor.findWordAt(textPos);
    let midPos = { line: 0, ch: 0 };
    if (wordPos.head.line == wordPos.anchor.line) {
      midPos = { line: wordPos.head.line, ch: (wordPos.head.ch + wordPos.anchor.ch) / 2 };
    } else {
      midPos = wordPos.head;
    }
    let thisTooltip = null;
    let txtInd = editor.indexFromPos(textPos);
    for (let i = 0; i < tooltips.length; i++) {
      if (tooltips[i].start <= txtInd && txtInd <= tooltips[i].end) {
        thisTooltip = tooltips[i];
        break;
      }
    }
    //Whether the cursor is outside the current word
    let outsideCurrentWord = (
      textPos.outside ||
      (
        (
          textPos.line > currentWordEnd.line ||
          (
            textPos.line == currentWordEnd.line &&
            textPos.ch > currentWordEnd.ch
          )
        ) ||
        (
          textPos.line < nextWordStart.line ||
          (
            textPos.line == nextWordStart.line &&
            textPos.ch < nextWordStart.ch
          )
        )
      )
    );
    let overWidget = (
      activeWidget != null &&
      (
        e.target == activeWidget ||
        activeWidget.contains(e.target)
      )
    );
    //Conditions for cancelling removal of a current tooltip
    if (
      //There is a plan to remove the current widget
      removeTimeout != null &&
      //There is an active widget
      activeWidget != null &&
      //Cursor is now inside the word again
      (
        !outsideCurrentWord ||
        overWidget
      )
    ) {
      clearTimeout(removeTimeout);
      removeTimeout = null;
    }
    //Conditions for cancelling adding of a new tooltip
    if (
      //There is a plan to add a widget
      displayTimeout != null &&
      //Text pos is outside the bounds of that new widget
      outsideCurrentWord
    ) {
      clearTimeout(displayTimeout);
      displayTimeout = null;
    }
    //Conditions for removing current tooltip
    if (
      //There is a current widget that isn't already being removed
      removeTimeout == null &&
      activeWidget != null &&
      (
        outsideCurrentWord &&
        //Cursor is not over the widget
        !overWidget
      )
    ) {
      removeTimeout = setTimeout(() => {
        clearWidget();
      }, 250);
    }
    //Conditions for adding a new tooltip
    if (
      //Not already trying to add one
      displayTimeout == null &&
      //There is a tooltip here
      thisTooltip != null &&
      //Cursor is not over an existing widget
      !overWidget
    ) {
      nextWordStart = wordPos.anchor;
      nextWordEnd = wordPos.head;
      displayTimeout = setTimeout(() => {
        currentWordStart = wordPos.anchor;
        currentWordEnd = wordPos.head;
        clearWidget();
        activeWidget = getNewTooltip(thisTooltip);
        lastWordPos = midPos;
        editor.addWidget(midPos, activeWidget);
      }, 500);
      if (removeTimeout != null) {
        clearTimeout(removeTimeout);
        removeTimeout = null;
      }
    }
  }

  editor.on("change", (cm, change) => {
    updateCode();
  });
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
  language_worker = new Worker(new URL("./language_worker.js", import.meta.url));
  language_worker.onmessage = e => {
    let command = e.data.command;
    let data = e.data.data;
    switch (command) {
      case "parsed":
        setup_runner();
        imports = data.imports;
        jscode.innerText = data.js;
        tooltips = JSON.parse(data.wordTriggers);
        let highlights = data.hl;
        editor.doc.getAllMarks().forEach(marker => marker.clear());
        for (let hl of highlights) {
          editor.markText(
            { line: hl.line, ch: hl.index },
            { line: hl.line, ch: hl.index + hl.length },
            { className: hl.color.at(-1) }
          );
        }
        pause();
        curr_frame = 0;
        runCode();
        play();
        break;
    }
  };
}

function setup_runner() {
  runner_worker?.terminate();
  runner_worker = new Worker(new URL("./runner_worker.js", import.meta.url));
  let function_dict = {
    "print_console": print_console,
    "bezier_point": bezier_point,
    "draw_bezier": draw_bezier,
    "draw_line": draw_line,
    "move_to": move_to,
    "rotate_delta": rotate_delta,
    "reset_rotation": reset_rotation,
    "draw_rect": draw_rect,
    "draw_ellipse": draw_ellipse,
    "set_stroke": set_stroke,
    "set_fill": set_fill,
    "set_line_width": set_line_width,
    "end_shape": end_shape,
  };
  runner_worker.onmessage = e => {
    let command = e.data.command;
    let data = e.data.data;
    switch (command) {
      case "finished":
        for (let funcCall of data) {
          function_dict[funcCall.name](...funcCall.args);
        }
        if (has_import(Import.Frame)) {
          print_console("fps:", Math.round(1000 / (Date.now() - last_frame_timestamp)));
        }
        last_frame_timestamp = Date.now();
        latest_frame = curr_frame;
        swap_canvases();
        break;
    }
  };
}

function has_import(imp) {
  return imports.indexOf(imp) >= 0;
}

function play() {
  pause();
  if (has_import(Import.Frame)) {
    last_frame_timestamp = Date.now();
    frameInterval = setInterval(draw_frame);
  }
}

function pause() {
  clearInterval(frameInterval);
}

function draw_frame() {
  let now = Date.now();
  if (latest_frame == curr_frame && (now - last_frame_timestamp) > 1000 / target_fps) {
    curr_frame++;
    runCode();
  }
}

function swap_canvases() {
  displayed_canvas.style.visibility = "hidden";
  curr_canvas.style.visibility = "visible";
  let temp_ctx = displayed_ctx;
  let temp_canvas = displayed_canvas;
  displayed_canvas = curr_canvas;
  displayed_ctx = curr_ctx;
  curr_ctx = temp_ctx;
  curr_canvas = temp_canvas;
}

window.pause = pause;
window.play = play;
window.openTab = openTab;
export default initialize;

