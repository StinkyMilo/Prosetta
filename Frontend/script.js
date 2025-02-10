import { allWords, wordsForAliases, partsOfSpeech } from './wordsForAliases.js';
import { Import } from './wasm-bindings/prosetta.js';
import { ALIAS_DATA } from './alias_data.js';

var jscode, sourcecode, cnsl, stack, curr_canvas, displayed_ctx, displayed_canvas, play_icon, pause_icon, toggle_btn, output_toggle_btn, primary, secondary;
/** @type CanvasRenderingContext2D
 */
var curr_ctx;
var x = 0, y = 0, rotation = 0;
var has_drawn_shape = false;
var last_shape = "none";
var language_worker, runner_worker;
/** @type CodeMirror.Editor */
var editor;
let tooltips = [];
/** @type int[] */
var imports = [];
var frameInterval;
var curr_frame = 0;
var latest_frame = 0;
var last_frame_timestamp = Date.now();
var target_fps = 30;
var actual_fps = 30;
var was_playing = true;
var currPath2D = null;
var version = 0;
/** @type Image
 */
var kirby_image = null;

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
  if (currPath2D != null) {
    curr_ctx.fill(currPath2D);
    curr_ctx.stroke(currPath2D);
    currPath2D = null;
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
  width = Math.abs(width);
  height = Math.abs(height);
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

function draw_star() {
  let diameter;
  switch (arguments.length) {
    case 1:
      diameter = arguments[0];
      break;
    case 3:
      _move_to(arguments[0], arguments[1]);
      diameter = arguments[2];
      break;
  }
  start_shape();
  // let x1, y1, x2, y2, x3, y3, x4, y4;
  let rad = rotation_radians()
  let [x1, y1] = rotate_point(x, y, rad, x, y - diameter / 2);
  curr_ctx.moveTo(x1, y1);
  for (let i = 1; i < 10; i++) {
    let x2, y2;
    if (i % 2 == 0) {
      [x2, y2] = rotate_point(x, y, rad + (i * 2 * Math.PI / 10), x, y - diameter / 2);
    }
    else {
      [x2, y2] = rotate_point(x, y, rad + (i * 2 * Math.PI / 10), x, y - diameter / 4);
    }
    curr_ctx.lineTo(x2, y2);
  }
  curr_ctx.closePath();
  last_shape = "star";
}

function draw_poly() {
  let diameter, sides;
  switch (arguments.length) {
    case 2:
      diameter = arguments[0];
      sides = arguments[1];
      break;
    case 4:
      _move_to(arguments[0], arguments[1]);
      diameter = arguments[2];
      sides = arguments[3];
      break;
  }
  diameter = Math.abs(diameter);
  sides = Math.abs(sides);
  sides = Math.floor(sides);
  start_shape();
  // let x1, y1, x2, y2, x3, y3, x4, y4;
  let rad = rotation_radians()
  let [x1, y1] = rotate_point(x, y, rad, x, y - diameter / 2);
  curr_ctx.moveTo(x1, y1);
  for (let i = 1; i < sides; i++) {
    let [x2, y2] = rotate_point(x, y, rad + (i * 2 * Math.PI / sides), x, y - diameter / 2);
    curr_ctx.lineTo(x2, y2);
  }
  curr_ctx.closePath();
  last_shape = "poly";
}

function draw_tri() { draw_poly(...arguments, 3); }
function draw_heart() {
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
  let xform = new DOMMatrix()
    .translate(x, y)
    .rotate(rotation)
    .translate(-width / 2, -height / 2)
    .scale(width / 100, height / 100);
  currPath2D = new Path2D();
  currPath2D.addPath(new Path2D("M 23.476563,3.295099 C 11.106703,3.2427741 0,11.644689 0,30.849786 0,57.679623 37.633692,72.409016 49.999998,96.70721 62.366306,72.409016 100,57.679623 100,30.849786 100,-3.2926084 64.896424,-3.293564 49.999998,17.654474 43.482815,8.4897074 33.097564,3.3357966 23.476563,3.295099 Z"), xform);
  last_shape = "heart";
}

function draw_round_rec() {
  let radius, width, height;
  switch (arguments.length) {
    case 2:
      width = arguments[0];
      height = arguments[0];
      radius = arguments[1];
      break;
    case 3:
      width = arguments[0];
      height = arguments[1];
      radius = arguments[2];
      break;
    case 4:
      _move_to(arguments[0], arguments[1]);
      width = arguments[2];
      height = arguments[2];
      radius = arguments[3];
      break;
    case 5:
      _move_to(arguments[0], arguments[1]);
      width = arguments[2];
      height = arguments[3];
      radius = arguments[4];
      break;
  }
  radius = Math.abs(radius);
  width = Math.abs(width);
  height = Math.abs(height);
  radius = Math.min(width / 2, height / 2, radius);
  start_shape();
  curr_ctx.save();
  curr_ctx.translate(x, y);
  curr_ctx.rotate(-rotation_radians());
  curr_ctx.roundRect(-width / 2, -height / 2, width, height, radius);
  curr_ctx.restore();
  last_shape = "round_rect";
}

async function draw_kirby() {
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
  curr_ctx.save();
  curr_ctx.translate(x, y);
  curr_ctx.rotate(-rotation_radians());
  curr_ctx.translate(-width / 2, -height / 2);
  if (kirby_image == null) {
    kirby_image = await new Promise((resolve, reject) => {
      const img = new Image();
      img.onload = () => resolve(img);
      img.onerror = (err) => reject(err);
      img.src = "icons/kirby.jpg";
    });
  }
  curr_ctx.drawImage(kirby_image, 0, 0, width, height);
  curr_ctx.restore();
  last_shape = "kirby";
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
  width = Math.abs(width);
  height = Math.abs(height);
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
}

function openTab(event, tab) {
  openTabGeneric("tabContent", "tabBtn", event, tab, "block");
}

function openTabGeneric(contentClassName, buttonClassName, event, tab, defaultDisplay) {
  let tabContents = document.getElementsByClassName(contentClassName);
  for (let i = 0; i < tabContents.length; i++) {
    if (tabContents[i].id == tab) {
      tabContents[i].style.display = defaultDisplay;
    }
    else {
      tabContents[i].style.display = "none";
    }
  }
  let tabs = document.getElementsByClassName(buttonClassName);
  for (let i = 0; i < tabs.length; i++) {
    tabs[i].className = tabs[i].className.replace(" active", "");
  }
  event.currentTarget.className += " active";
}

function updateCode() {
  if (editor == null) {
    return;
  }
  version++;
  last_frame_timestamp = Date.now();
  const src = editor.getValue();
  const replace_chars = {
    "–": "--",
    "—": "---",
    "…": "...",
    "“": '"',
    "”": '"',
    "‘": "'",
    "’": "'",
  }
  let newsrc = src;
  for (const [key, val] of Object.entries(replace_chars)) {
    newsrc = newsrc.replaceAll(key, val);
  }
  if (newsrc != src) {
    editor.setValue(newsrc);
    return;
  }
  console.log(src);
  const codeUpdateEvent = new CustomEvent("codeChanged", { detail: src });
  document.dispatchEvent(codeUpdateEvent);
  msg_worker("changed", src);
}

export async function initialize(startingCode) {
  for (const tooltip of document.getElementsByClassName("tooltip")) {
    tooltip.remove();
  }
  version = 0;
  language_worker?.terminate();
  primary = document.getElementById("primary");
  secondary = document.getElementById("secondary");
  output_toggle_btn = document.getElementById("output-toggle");
  sourcecode = document.getElementById("code");
  jscode = document.getElementById("js");
  stack = document.getElementById("stack");
  curr_canvas = document.getElementById("outputcanvas");
  displayed_canvas = document.getElementById("outputcanvas2");
  curr_ctx = curr_canvas.getContext('2d');
  displayed_ctx = displayed_canvas.getContext('2d');
  toggle_btn = document.getElementById("toggle-play");
  play_icon = document.getElementById("play-icon");
  pause_icon = document.getElementById("pause-icon");
  jscode.innerText = "";
  cnsl = document.getElementById("console");
  showing_canvas = true;
  update_output();

  init_canvas();
  print_console("Welcome to Prosetta!");
  print_console("---");
  print_console();
  let editor = setup_editor();
  setup_lang_worker();
  editor.setValue(startingCode);
  return editor;
}


function msg_worker(command, data) {
  language_worker.postMessage({ command: command, data: data });
}

function getWordsOfLength(len, mod10) {
  if (mod10) {
    return allWords.filter((word) => {
      return word.length % 10 == len;
    })
  }
  return allWords.filter((word) => {
    return word.length == len;
  });
}

function getWordsThatContain(substr) {
  return allWords.filter((word) => {
    return word.indexOf(substr.toLowerCase()) > -1;
  });
}

const BASE_URL = "https://stinkymilo.github.io/Prosetta/Frontend/docs/#/"
function setup_editor() {
  let code = document.getElementById("code");
  while (code.hasChildNodes()) {
    code.removeChild(code.firstChild);
  }
  editor = CodeMirror(code, {
    value: "",
    mode: "plaintext",
    lineWrapping: true,
    lineNumbers: true,
    theme: "xq-dark"
  });
  editor.on("keydown", key_press_handler);
  editor.setSize("100%", "100%");

  const PARTS_OF_SPEECH = ["noun", "verb", "adjective", "adverb", "other"];
  const BASE_URL = "https://stinkymilo.github.io/Prosetta/Frontend/docs/#/"
  const BASE_URL_IMPORTS = BASE_URL + "Imports#";
  const IMPORTS = {
    "fram": "Animation",
    "fun": "Functions",
    "gra": "Graphics",
    "lis": "Lists",
    "ran": "Randomization",
    "tam": "Stamps",
    "tri": "Trigonometry",
  };
  /*
    Returns a node that contains the alternate word suggestions
  */
  function getNewTooltip(tooltip) {
    //For now, don't use rust endpoints; just choose the first alias.
    //Later, we'll want to use the rust endpoints though
    let widget = document.createElement("div");
    widget.className = "tooltip";
    let header = document.createElement("h1");
    let span = document.createElement("span");
    header.appendChild(span);
    let words;
    if (tooltip.type == "alias") {
      words = wordsForAliases[tooltip.value];
      span.innerText = `Words that trigger`
      header.insertAdjacentHTML("beforeend",
        `<a href='${BASE_URL}${ALIAS_DATA[tooltip.value].url}' rel='noopener noreferrer'\
        target='_blank'>🔗${tooltip.value} (${ALIAS_DATA[tooltip.value].name})</a>`);
    } else if (tooltip.type == "length") {
      words = getWordsOfLength(tooltip.len, tooltip.mod10);
      if (tooltip.mod10) {
        span.innerHTML =
          `Words of length <span class='term_b_green'>${tooltip.len}</span>, <span class='term_b_green'>${tooltip.len + 10}</span> etc.`;
      } else {
        span.innerHTML =
          `Words of length <span class='term_b_green'>${tooltip.len}</span>`;
      }
    } else if (tooltip.type == "variable") {
      words = getWordsThatContain(tooltip.name);
      span.innerHTML = `Words that contain the variable <span class='term_b_blue'>${tooltip.name}</span>`;
    } else if (tooltip.type == "import") {
      words = [];
      span.innerHTML = `Import: <a href='${BASE_URL_IMPORTS + tooltip.name}-${IMPORTS[tooltip.name].toLowerCase()}' rel='noopener noreferrer' target='_blank'>${IMPORTS[tooltip.name]}</a> Library`;
    }

    let buttonContainer = document.createElement("div");
    buttonContainer.className = "posTabGroup";
    let tabContainer = document.createElement("div");
    let tabContents = {};
    for (let i = 0; i < PARTS_OF_SPEECH.length; i++) {
      let pos = PARTS_OF_SPEECH[i];
      let posTabButton = document.createElement("button");
      let posTabContent = document.createElement("div");
      posTabContent.id = pos;
      posTabButton.innerHTML = pos;
      posTabButton.className = "posTabBtn";
      if (i == 0) {
        posTabButton.className += " active";
        posTabContent.style.display = "flex";
      } else {
        posTabContent.style.display = "none";
      }
      posTabContent.className = "posTabContent";
      posTabButton.onclick = (e) => {
        console.log("Button clicked");
        openTabGeneric("posTabContent", "posTabBtn", e, pos, "flex");
      }
      buttonContainer.appendChild(posTabButton);
      tabContainer.appendChild(posTabContent);
      tabContents[pos] = posTabContent;
    }
    let closeButton = document.createElement("button");
    closeButton.innerHTML = `
    <svg style="width: 10px; height: 10px; margin: 2px; margin: 0; padding: 0; padding-bottom: 0px; padding-bottom: 2.5px;"
   width="25"
   height="25"
   viewBox="0 0 6.6145832 6.6145833"
   version="1.1"
   id="svg1"
   xmlns="http://www.w3.org/2000/svg"
   xmlns:svg="http://www.w3.org/2000/svg">
  <defs
     id="defs1" />
  <g
     id="layer1"
     style="fill:#ffffff;fill-opacity:1">
    <path
       id="path4"
       style="stroke-width:0.199999;fill:#ffffff;fill-opacity:1"
       d="M 0 0 L 0 1.3229166 L 5.2916666 6.6145832 L 6.6145832 6.6145832 L 6.6145832 5.2916666 L 1.3229166 0 L 0 0 z " />
    <path
       id="path3"
       style="stroke-width:0.200243;fill:#ffffff;fill-opacity:1"
       d="M 5.3614298 0 L 0 5.2167358 L 0 6.6145832 L 1.2531535 6.6145832 L 6.6145832 1.3978475 L 6.6145832 0 L 5.3614298 0 z " />
  </g>
</svg>`;
    // xImg.src = "icons/x.svg";
    // xImg.style.width = "100%";
    // xImg.style.height = "100%";
    closeButton.className = "close-button";
    closeButton.onclick = clearWidget;
    //Start adding to widget directly.
    widget.appendChild(header);
    widget.appendChild(closeButton);
    if (tooltip.type != "import") {
      widget.appendChild(buttonContainer);
      widget.appendChild(tabContainer);
      for (let i = 0; i < words.length; i++) {
        let matchingPos = partsOfSpeech[words[i]];
        for (let j = 0; j < matchingPos.length; j++) {
          let wordElement = document.createElement("div");
          wordElement.innerHTML = words[i];
          wordElement.onclick = () => {
            editor.replaceRange(words[i], currentWordStart, currentWordEnd);
            currentWordEnd = { line: currentWordStart.line, ch: currentWordStart.ch + words[i].length };
          };
          tabContents[matchingPos[j]].appendChild(wordElement);
        }
      }
    } else {
      widget.style.height = "auto";
    }
    return widget;
  }

  let activeWidget = null;
  let lastWordPos = { line: -1, ch: -1 };
  let displayTimeout;
  let removeTimeout;
  let currentWordStart = { line: -1, ch: -1 };
  let currentWordEnd = { line: -1, ch: -1 };
  let nextWordStart = { line: -1, ch: -1 };
  let nextWordEnd = { line: -1, ch: -1 };

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

  window.onmousemove = function (e) {
    let pos = { left: e.clientX, top: e.clientY + window.scrollY };
    let textPos = editor.coordsChar(pos);
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
      if (tooltips[i].start <= txtInd && txtInd < tooltips[i].end) {
        thisTooltip = tooltips[i];
        break;
      }
    }
    const isMouseInCode = sourcecode.contains(e.target) || (activeWidget != null && activeWidget.contains(e.target));
    //Whether the cursor is outside the current word
    let outsideCurrentWord = (
      !isMouseInCode ||
      (textPos.outside ||
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
      !isMouseInCode ||
      //There is a plan to add a widget
      (displayTimeout != null &&
        //Text pos is outside the bounds of that new widget
        outsideCurrentWord)
    ) {
      clearTimeout(displayTimeout);
      displayTimeout = null;
    }
    //Conditions for removing current tooltip
    if (
      !isMouseInCode ||
      //There is a current widget that isn't already being removed
      (removeTimeout == null &&
        activeWidget != null &&
        (
          outsideCurrentWord &&
          //Cursor is not over the widget
          !overWidget
        ))
    ) {
      removeTimeout = setTimeout(() => {
        clearWidget();
      }, 250);
    }
    //Conditions for adding a new tooltip
    if (
      isMouseInCode &&
      (//Not already trying to add one
        displayTimeout == null &&
        //There is a tooltip here
        thisTooltip != null &&
        //Cursor is not over an existing widget
        !overWidget
      )
    ) {
      nextWordStart = wordPos.anchor;
      nextWordEnd = wordPos.head;
      displayTimeout = setTimeout(() => {
        currentWordStart = wordPos.anchor;
        currentWordEnd = wordPos.head;
        clearWidget();
        activeWidget = getNewTooltip(thisTooltip);
        document.body.appendChild(activeWidget);
        activeWidget.style.position = "absolute";
        activeWidget.style.left = e.pageX + "px";
        activeWidget.style.top = e.pageY + "px";
        console.log(activeWidget);
        lastWordPos = midPos;
        // editor.addWidget(midPos, activeWidget);
        // sourcecode.appendChild(activeWidget);
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



function try_add_autocomplete() {
  remove_auto_tooltip();
  let pos = editor.getCursor();
  let txt_index = editor.indexFromPos(pos);
  let thisTooltip = null;
  // look for start index in tooltip array
  for (let i = 0; i < tooltips.length; i++) {
    if (tooltips[i].start < txt_index && txt_index <= tooltips[i].end) {
      thisTooltip = tooltips[i];
      break;
    }
  }

  if (thisTooltip && thisTooltip.type == "alias" && !thisTooltip.has_matched) {
    add_autocomplete(thisTooltip.value, pos);
  }
}


let autocomplete_widget = null;
let autocomplete_index = null;
let autocomplete_usage = null;
let autocomplete_position = null;

function add_autocomplete(tooltip_name, pos) {
  let data = ALIAS_DATA[tooltip_name];

  autocomplete_position = pos;
  autocomplete_usage = data.usage;

  if (autocomplete_usage.length) {
    autocomplete_index = 0;
    autocomplete_widget = document.createElement("div");
    autocomplete_widget.className = "auto-tooltip"

    for (let index = 0; index < autocomplete_usage.length; index++) {
      let line = document.createElement("div");

      let link = document.createElement("a");
      link.innerText = "🔗"
      link.href = `${BASE_URL}${data.url}?id=${autocomplete_usage[index].id}`;
      line.appendChild(link);

      let format = document.createElement("span");
      format.innerText = " " + autocomplete_usage[index].format;
      format.onclick = () => handle_autocomplete(index);
      line.appendChild(format);

      line.className = index == 0 ? "selected" : "";
      autocomplete_widget.appendChild(line);
    }

    document.body.appendChild(autocomplete_widget);
    let coords = editor.charCoords(pos);
    autocomplete_widget.style.position = "absolute";
    autocomplete_widget.style.left = coords.left + "px";
    autocomplete_widget.style.top = coords.bottom + "px";
  }
}

function remove_auto_tooltip() {
  if (autocomplete_widget) {
    document.body.removeChild(autocomplete_widget);
    autocomplete_widget = null;
  }
}

function handle_autocomplete(index) {
  editor.replaceRange(" " + autocomplete_usage[index].func(), autocomplete_position, autocomplete_position)
  remove_auto_tooltip();
  updateCode();
}

function change_autocomplete(offset) {
  autocomplete_widget.children[autocomplete_index].className = "";
  autocomplete_index = ((autocomplete_index + offset + autocomplete_usage.length) % autocomplete_usage.length);
  autocomplete_widget.children[autocomplete_index].className = "selected";
}

function key_press_handler(cm, ev) {
  if (autocomplete_widget) {
    if (ev.key == "ArrowUp") {
      ev.preventDefault();
      change_autocomplete(-1);
    }
    if (ev.key == "ArrowDown" || ev.key == "Enter") {
      ev.preventDefault();
      change_autocomplete(1);
    }
    if (ev.key == "Tab") {
      ev.preventDefault();
      handle_autocomplete(autocomplete_index)
    }
  }
}

function setup_lang_worker() {
  language_worker?.terminate();
  version = 0;
  language_worker = new Worker(new URL("./language_worker.js", import.meta.url));
  language_worker.postMessage({ command: "initialize" });
  language_worker.onmessage = e => {
    let command = e.data.command;
    let data = e.data.data;
    switch (command) {
      case "parsed":
        if (version != data.version) {
          break;
        }
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
        try_add_autocomplete();

        const doesPrint = tooltips.filter(x => x.type == "alias").map(x => x.value).some(x => x == "pri") || imports.find(x => x == Import.Frame) != undefined;
        const doesDraw = imports.find(x => x == Import.Graph) != undefined || imports.find(x => x == Import.Stamp) != undefined;
        if (doesDraw && doesPrint) {
          primary.appendChild(stack);
          secondary.style.display = "inherit";
          secondary.appendChild(cnsl);
        }
        else if (doesDraw) {
          primary.appendChild(stack);
          secondary.style.display = "none";
          secondary.appendChild(cnsl);
        }
        else {
          primary.appendChild(cnsl);
          secondary.style.display = "none";
          secondary.appendChild(stack);
        }

        pause();
        curr_frame = 0;
        runCode();
        play();
        break;
    }
  };
  if (editor) {
    editor.doc.getAllMarks().forEach(marker => marker.clear());
    editor.setValue(editor.getValue());
  }
}

function setup_runner() {
  runner_worker?.terminate();
  runner_worker = new Worker(new URL("./runner_worker.js", import.meta.url));
  let function_dict = {
    "print_console": print_console,
    "bezier_point": bezier_point,
    "draw_bezier": draw_bezier,
    "draw_line": draw_line,
    "draw_star": draw_star,
    "draw_poly": draw_poly,
    "draw_tri": draw_tri,
    "draw_heart": draw_heart,
    "draw_round_rec": draw_round_rec,
    "draw_kirby": draw_kirby,
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
  runner_worker.onmessage = async e => {
    let command = e.data.command;
    let data = e.data.data;
    switch (command) {
      case "finished":
        for (let funcCall of data) {
          await function_dict[funcCall.name](...funcCall.args);
        }
        if (has_import(Import.Frame)) {
          actual_fps = 1000 / (Date.now() - last_frame_timestamp);
          print_console("fps:", Math.round(actual_fps));
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

var isPlaying = true;
function toggle() {
  if (frameInterval) {
    pause();
  } else {
    play();
  }
}

function play() {
  pause();
  play_icon.style.display = "none";
  pause_icon.style.display = "block";
  if (has_import(Import.Frame)) {
    toggle_btn.style.display = "block"
    last_frame_timestamp = Date.now();
    isPlaying = true;
    draw_frame();
  }
}

function pause() {
  play_icon.style.display = "block";
  pause_icon.style.display = "none";
  isPlaying = false;
}

function draw_frame() {
  let now = Date.now();
  if (latest_frame == curr_frame && (now - last_frame_timestamp + 10) >= 1000 / target_fps) {
    curr_frame++;
    runCode();
  }
  if (isPlaying) {
    requestAnimationFrame(draw_frame);
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

function update_output() {
  if (showing_canvas) {
    output_toggle_btn.style.backgroundColor = "lightGrey";
    output_toggle_btn.children[0].style.webkitTransform = "translateX(0px)";
    output_toggle_btn.children[0].style.msTransform = "translateX(0px)";
    output_toggle_btn.children[0].style.transform = "translateX(0px)";
    primary.style.display = "block";
    jscode.style.display = "none";
    if (was_playing) {
      play();
    }
  } else {
    output_toggle_btn.style.backgroundColor = "black";
    output_toggle_btn.children[0].style.webkitTransform = "translateX(26px)";
    output_toggle_btn.children[0].style.msTransform = "translateX(26px)";
    output_toggle_btn.children[0].style.transform = "translateX(26px)";
    primary.style.display = "none";
    jscode.style.display = "block";
    was_playing = !!frameInterval;
    pause();
  }
}

export function updateValue(newValue) {
  editor.setValue(newValue);
}

var showing_canvas = true;
function toggle_canvas() {
  showing_canvas = !showing_canvas;
  update_output();
}



window.reset = setup_lang_worker;
window.toggle = toggle;
window.toggle_canvas = toggle_canvas;
window.update_output = update_output;

export default { initialize, updateValue };
