import init, { ParserRunner } from './wasm-bindings/prosetta.js'

var jscode, sourcecode, syntax, ctx, cnsl, canvas;
var x = 0, y = 0, rotation = 0;
var has_run = false;
var has_drawn_shape = false;
var last_shape = "none";
var parser, parsedData;
var old_code;
var editor;

function init_canvas() {
  sourcecode = document.getElementById("code");
  jscode = document.getElementById("js");
  syntax = document.getElementById("syntax");
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
  let new_code = editor.getValue();
  if (new_code == old_code) {
    return;
  }
  old_code = new_code;
  parsedData = parser.run_to_completion(new_code);
  jscode.innerHTML = parsedData.get_javascript();
  syntax.innerHTML = parsedData.get_html();
  let c = syntax.children;
  // for (let i = 0; i < c.length; i++) {
  //   c[i].style.color = c[i].className.substring("term_b_".length, c[i].className.length);
  // }
}

document.addEventListener("DOMContentLoaded", () => {
  initialize();
});

async function initialize() {
  let tabs = document.getElementsByClassName("tabBtn tabDefault");
  tabs[0].click();

  await init();
  parser = new ParserRunner();
  init_canvas();
  print_console("Welcome to Prosetta!");
  print_console("---");
  print_console();
  old_code = "";
  updateCode();
}

window.runCode = runCode;
window.updateCode = updateCode;
window.openTab = openTab;

import CodeMirror from 'codemirror';
editor = CodeMirror(document.getElementById("code"), {
  value: "Draw a rectangle around my thirty fifty dollar bills!",
  mode: "plaintext"
});

const worker = new Worker('lsp-worker.js');

// const connection = {
//   on: (e) => console.log(e),
//   getHoverTooltip: (e) => {
//     worker.postMessage({
//       jsonrpc: "2.0",
//       id: 1,
//       method: 'textDocument/hover',
//       params: {
//         textDocument: "",
//         position: { line: e.line, character: e.ch }
//       }
//     });
//   },
//   send: (message) => worker.postMessage(message),
//   onNotification: (cb) => worker.onmessage = (event) => cb(event.data),
//   onRequest: (cb) => worker.onmessage = (event) => cb(event.data),
// };

import { CodeMirrorAdapter } from 'lsp-codemirror';
import * as marked from 'marked';
import { MarkupContent } from 'vscode-languageserver-protocol';
import LspWwConnection from './lsp-connection.js';
const connection = new LspWwConnection({
  serverUri: 'prosetta/lsp',
  mode: 'plaintext',
  rootUri: `file:///`,
  documentUri: `file:///poem.txt`,
  documentText: () => editor.getValue(),
}).connect(worker);
const lspAdapter = new CodeMirrorAdapter(connection, { quickSuggestionsDelay: 200 }, editor);
lspAdapter._showTooltip = function(el, coords) {
  if (this.isShowingContextMenu) {
    this._removeTooltip();
  }

  let top = coords.y;

  this.tooltip = document.createElement('div');
  this.tooltip.classList.add('CodeMirror-lsp-tooltip');
  this.tooltip.style.left = `${coords.x}px`;
  this.tooltip.style.top = `${top}px`;
  this.tooltip.appendChild(el);
  document.body.appendChild(this.tooltip);

  // Measure and reposition after rendering first version
  requestAnimationFrame(() => {
    this.tooltip.style.left = `${coords.x}px`;
    this.tooltip.style.top = `${top}px`;
  });

  this.isShowingContextMenu = true;
}.bind(lspAdapter);
lspAdapter.connection.off('hover', lspAdapter.handleHover);
lspAdapter.handleHover = function(response) {
  this._removeHover();
  this._removeTooltip();

  if (!response || !response.contents || (Array.isArray(response.contents) && response.contents.length === 0)) {
    return;
  }

  let start = this.hoverCharacter;
  let end = this.hoverCharacter;
  if (response.range) {
    start = {
      line: response.range.start.line,
      ch: response.range.start.character,
    };
    end = {
      line: response.range.end.line,
      ch: response.range.end.character,
    };

    this.hoverMarker = this.editor.getDoc().markText(start, end, {
      className: 'CodeMirror-lsp-hover'
    });
  }

  let tooltipText;
  const htmlElement = document.createElement('div');
  if (MarkupContent.is(response.contents)) {
    tooltipText = response.contents.value;

    htmlElement.innerHTML = marked.parse(tooltipText);
  } else {
    if (Array.isArray(response.contents)) {
      const firstItem = response.contents[0];
      if (MarkupContent.is(firstItem)) {
        tooltipText = firstItem.value;
      } else if (firstItem === null) {
        return;
      } else if (typeof firstItem === 'object') {
        tooltipText = firstItem.value;
      } else {
        tooltipText = firstItem;
      }
    } else if (typeof response.contents === 'string') {
      tooltipText = response.contents;
    }

    htmlElement.innerText = tooltipText;
  }
  const coords = this.editor.charCoords(start, 'page');
  this._showTooltip(htmlElement, {
    x: coords.left,
    y: coords.top,
  });
}.bind(lspAdapter)
lspAdapter.connection.on('hover', lspAdapter.handleHover);
// lspAdapter._addListeners();
connection.sendInitialize()
