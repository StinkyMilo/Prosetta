function random(min, max = NaN, step = NaN) {
    if (isNaN(max)) {
        max = min;
        min = 0;
    }
    if (isNaN(step)) {
        step = 1;
    } else {
        max += step;
    }
    return Math.floor(Math.random() * (max - min) / step + min / step) * step;
}

/**
 * @return {number} between min and max (inclusive)
 */
function random_num_inclusive(min, max) {
    return random(min, max, 1)
}

function random_digit() {
    return random(0, 10)
}

function random_digit_neg() {
    return random(-9, 9, 1)
}

function random_byte() {
    return random(255)
}

function random_angle() {
    return random(15, 90, 15)
}

function random_trig_angle() {
    return random(-90, 270, 15)
}


function random_size() {
    return random(10, 90, 10)
}


function random_location() {
    return random(-75, 75, 25)
}

function random_sides() {
    return random(3, 8, 1)
}


function random_list(min_start, length = 3) {
    let start = random_num_inclusive(min_start, 10 - length);
    let arr = [...new Array(length).keys()].map((_, i) => i + start)
    return arr
}
//https://www.geeksforgeeks.org/3-letter-words/
let VAR_NAMES =
    ['and', 'fix', 'own', 'are', 'fly', 'odd', 'ape', 'fry',
        'our', 'ace', 'for', 'pet', 'act', 'got', 'pat', 'ask',
        'get', 'peg', 'arm', 'god', 'paw', 'age', 'gel', 'pup',
        'ago', 'gas', 'pit', 'air', 'hat', 'put', 'ate', 'hit',
        'pot', 'all', 'has', 'pop', 'but', 'had', 'pin', 'bye',
        'how', 'rat', 'bad', 'her', 'rag', 'big', 'his', 'rub',
        'bed', 'hen', 'row', 'bat', 'ink', 'rug', 'boy', 'ice',
        'run', 'bus', 'ill', 'rap', 'bag', 'jab', 'ram', 'box',
        'jug', 'sow', 'bit', 'jet', 'see', 'bee', 'jam', 'saw',
        'buy', 'jar', 'set', 'bun', 'job', 'sit', 'cub', 'jog',
        'sir', 'cat', 'kit', 'sat', 'car', 'key', 'sob', 'cut',
        'lot', 'tap', 'cow', 'tip', 'cry', 'let', 'top',
        'cab', 'lay', 'tug', 'can', 'mat', 'tow', 'dad', 'man',
        'toe', 'dab', 'mad', 'dam', 'mug', 'did',
        'mix', 'dug', 'map', 'use', 'den', 'mum', 'van',
        'dot', 'mud', 'vet', 'dip', 'mom', 'day', 'may',
        'wet', 'ear', 'met', 'win', 'eye', 'net', 'won', 'eat',
        'new', 'wig', 'end', 'nap', 'war', 'elf', 'now', 'why',
        'egg', 'nod', 'who', 'far', 'net', 'way', 'fat',
        'wow', 'few', 'nut', 'you', 'fan', 'oar', 'yes',
        'yak', 'fit', 'out', 'yet', 'fin', 'owl', 'zip',
        'fox', 'old', 'zap'];
function random_var_name() {
    return VAR_NAMES[random(VAR_NAMES.length)]
}

function random_color() {
    let colors = ["blue", "cyan", "gold", "gray", "grey", "lime", "navy", "pink", "plum", "snow", "teal"];
    return colors[random_num_inclusive(0, 5)];
}

function random_bool() {
    return colors[random_num_inclusive(0, 1)];
}

function random_bool_equation() {
    let num1, num2, op;
    switch (random(3)) {
        case 0:
            op = "par"
            num2 = num1 = random(0, 10);
            break;
        case 1:
            op = "mor"
            num1 = random(1, 10);
            num2 = random(0, num1);
            break;
        case 2:
            op = "les"
            num1 = random(0, 9);
            num2 = random(num1 + 1, 10);
            break;
    }
    return `${op} ${num1} ${num2}.`;
}

function random_math() {
    let num1, num2, op;
    switch (random(4)) {
        case 0:
            op = "add"
            num1 = random(1, 10);
            num2 = random(1, 10);
            break;
        case 1:
            op = "sub"
            num1 = random(1, 10);
            num2 = random(1, 10);
            break;
        case 2:
            op = "tim"
            num1 = random(2, 5);
            num2 = random(2, 5);
            break;
        case 3:
            op = "ide"
            num2 = random_num_inclusive(2, 5);
            let val = random_num_inclusive(2, 5);
            num1 = val * num2
            break;
    }
    return `${op} ${num1} ${num2}.`;
}


/*
interface AliasData {
    url: string,
    usage: Usage
}

interface Usage {
    id: string,
    format: string,
    func: () => string
}
*/

export const ALIAS_DATA = {
    "abs": {
        url: "AbsoluteValue",
        name: "Absolute Value",
        usage: [
            {
                id: "arguments",
                format: "value (number)",
                func: () => `${random_digit_neg()}. `
            }
        ]
    },
    "add": {
        url: "Add",
        name: "Add",
        usage: [
            {
                id: "arguments",
                format: "num_1 num_2 [num_3 ... num_∞]",
                func: () => `${random_digit()} ${random_digit()} . `
            }
        ]
    },
    "als": {
        url: "And",
        name: "And",
        usage: [
            {
                id: "arguments",
                format: "cond_1, cond_2, [cond3 ... cond_∞] (booleans or numbers)",
                func: () => `${random_bool()} ${random_bool()} . `
            }
        ]

    },
    "app": {
        url: "Append",
        name: "Append",
        usage: [
            {
                id: "end-of-list",
                format: "list (list), value (any)",
                func: () => {
                    let list = random_list(2, 4);
                    let item = list.pop();
                    return `lis ${list.join(' ')} . ${item}. `
                }
            },
            {
                id: "specified-location",
                format: "list (list), value (any), index (number)",
                func: () => {
                    let list = random_list(2, 4);
                    let index = random(0, list.length);
                    let item = list.splice(index, 1)[0];
                    return `lis ${list.join(' ')} . ${item} ${index}. `
                }
            }
        ]
    },
    "arc": {
        url: "Ellipse",
        name: "Ellipse",
        usage: [
            {
                id: "circle-relative",
                format: "diameter (number)",
                func: () => `${random_size()}. `
            },
            {
                id: "ellipse-relative",
                format: "width (number), height (number)",
                func: () => `${random_size()} ${random_size()}. `
            },
            {
                id: "circle-absolute",
                format: "x (number), y (number), diameter (number)",
                func: () => `${random_location()} ${random_location()} ${random_size()}. `
            },
            {
                id: "ellipse-absolute",
                format: "x (number), y (number), width (number), height (number)",
                func: () => `${random_location()} ${random_location()} ${random_size()} ${random_size()}. `
            }
        ]
    },
    "bez": {
        url: "Bezier",
        name: "Bezier",
        usage: [
            {
                id: "arguments",
                format: "x_1, y_1, x_2, y_2, [x_3, y_3, ... x_∞, y_∞] (numbers)",
                func: () => `${random_location()} ${random_location()} ${random_location()} ${random_location()} . `
            }
        ]
    },
    "col": {
        url: "Color",
        name: "Color",
        usage: [
            {
                id: "arguments",
                format: "r (number), g (number), b (number)",
                func: () => `${random_byte()} ${random_byte()} ${random_byte()}. `
            }
        ]
    },
    "cos": {
        url: "Cosine",
        name: "Cosine",
        usage: [
            {
                id: "arguments",
                format: "value (number)",
                func: () => `${random_trig_angle()}. `
            }
        ]
    },
    "cou": {
        url: "Length",
        name: "Length",
        usage: [
            {
                id: "arguments",
                format: "value (number)",
                func: () => `lis ${random_list(2, random_num_inclusive(2, 3)).join(" ")} .. `
            }
        ]
    },
    "del": {
        url: "Delete",
        name: "Delete",
        usage: [
            {
                id: "start-of-list",
                format: "list (list)",
                func: () => {
                    let list = random_list(3, 3);
                    let index = 0;
                    list.splice(index, 0, 1);
                    return `lis ${list.join(' ')} .. `
                }
            },
            {
                id: "specified-location",
                format: "list (list), index (number)",
                func: () => {
                    let list = random_list(3, 3);
                    let index = random(0, list.length);
                    list.splice(index, 0, 1);
                    return `lis ${list.join(' ')} . ${index}.`
                }
            }
        ]
    },
    "els": {
        url: "Else",
        name: "Else",
        usage: [
            {
                id: "arguments",
                format: "st_1, [st_2 ... st_∞] (statements)",
                func: () => `\n[...]\n. `
            }
        ]
    },
    "exp": {
        url: "Exponentiate",
        name: "Exponentiate",
        usage: [
            {
                id: "ex",
                format: "exp (number)",
                func: () => `${1}. `
            },
            {
                id: "standard",
                format: "base (number), exp (number)",
                func: () => `${random_num_inclusive(2, 5)} ${random_num_inclusive(2, 3)}. `
            }
        ]
    },
    "fil": {
        url: "Fill",
        name: "Fill",
        usage: [
            {
                id: "color",
                format: "color (color)",
                func: () => `${random_color()}.`
            },
            {
                id: "rgb",
                format: "red (number), green (number), blue (number)",
                func: () => `${random_byte()} ${random_byte()} ${random_byte()}. `
            },
        ]
    },
    "fin": {
        url: "Find",
        name: "Find",
        usage: [
            {
                id: "arguments",
                format: "toSearch (list or string), value (any)",
                func: () => {
                    let list = random_list(2);
                    let index = random(list.length);
                    let item = list[index];
                    return `lis ${list.join("")}. ${item}.`;
                }
            }
        ]
    },
    "flo": {
        url: "Floor",
        name: "Floor",
        usage: [
            {
                id: "rgb",
                format: "red (number), green (number), blue (number)",
                func: () => `${random_byte()} ${random_byte()} ${random_byte()}. `
            }
        ]
    },
    "fra": {
        url: "Frame",
        name: "Frame",
        usage: [
            {
                id: "fra-frame",
                format: "None",
                func: () => `. `
            }
        ]
    },
    "fre": {
        url: "Foreach",
        name: "For Each",
        usage: [
            {
                id: "",
                format: "name (word), list (list), [st_1, ... st_∞] (statements)",
                func: () => `${random_var_name()} lis ${random_list(1).join(" ")}.\n[...]\n. `
            },
            {
                id: "",
                format: "name (word), range (number), [st_1, ... st_∞] (statements)",
                func: () => `${random_var_name()} ${random(1, 10)}\n[...]\n. `
            }
        ]
    },
    "fun": {
        url: "Function",
        name: "Function",
        usage: [
            {
                id: "arguments",
                format: "name (word), [arg_1, ... arg_∞] (words), [CLOSING PUNCTUATION], [st_1, ... st_∞] (statements)",
                func: () => `${random_var_name()} ${random_var_name()}.\n[...]\n. `
            }
        ]
    },
    "hea": {
        url: "Heart",
        name: "Heart",
        usage: [
            {
                id: "relative-regular",
                format: "size (number)",
                func: () => `${random_size()}. `
            },
            {
                id: "relative-stretched",
                format: "width (number), height (number)",
                func: () => `${random_size()} ${random_size()}. `
            },
            {
                id: "absolute-regular",
                format: "x (number), y (number), size (number)",
                func: () => `${random_location()} ${random_location()} ${random_size()}. `
            },
            {
                id: "absolute-stretched",
                format: "x (number), y (number), width (number), height (number)",
                func: () => `${random_location()} ${random_location()} ${random_size()} ${random_size()}. `
            }
        ]
    },
    "ide": {
        url: "Divide",
        name: "Divide",
        usage: [
            {
                id: "arguments",
                format: "num_1 num_2",
                func: () => {
                    let val1 = random_num_inclusive(2, 5);
                    let val2 = random_num_inclusive(2, 5);
                    return `${val1 * val2} ${val1}. `
                }
            }
        ]
    },
    "ind": {
        url: "Index",
        name: "Index",
        usage: [
            {
                id: "arguments",
                format: "list (list), index (number)",
                func: () => {
                    let list = random_list(2);
                    let index = random(list.length);
                    return `lis ${list.join(" ")}. ${index}. `;
                }
            }
        ]
    },
    "int": {
        url: "Int",
        name: "Small Number",
        usage: [
            {
                id: "arguments",
                format: "word (word)",
                func: () => `${random_var_name()}. `
            }
        ]
    },
    "inv": {
        url: "Not",
        name: "Not",
        usage: [
            {
                id: "arguments",
                format: "x (boolean or number)",
                func: () => `${random_bool()}. `
            }
        ]
    },
    "kir": {
        url: "Kirby",
        name: "Kirby",
        usage: [
            {
                id: "relative-regular",
                format: "size (number)",
                func: () => `${random_size()}. `
            },
            {
                id: "relative-stretched",
                format: "width (number), height (number)",
                func: () => `${random_size()} ${random_size()}. `
            },
            {
                id: "absolute-regular",
                format: "x (number), y (number), size (number)",
                func: () => `${random_location()} ${random_location()} ${random_size()}. `
            },
            {
                id: "absolute-stretched",
                format: "x (number), y (number), width (number), height (number)",
                func: () => `${random_location()} ${random_location()} ${random_size()} ${random_size()}. `
            }
        ]
    },
    "les": {
        url: "LessThan",
        name: "Less Than",
        usage: [
            {
                id: "arguments",
                format: "n1 (number), n2 (number)",
                func: () => {
                    let num1 = random(1, 9);
                    let num2 = random(0, num1);
                    return `${num1} ${num2}. `
                }
            }
        ]
    },
    "lin": {
        url: "Line",
        name: "Line",
        usage: [
            {
                id: "relative-angled",
                format: "length (number)",
                func: () => `${random_size()}. `
            },
            {
                id: "relative-x-y",
                format: "xMove (number), yMove (number)",
                func: () => `${random_size()} ${random_size()}. `
            },
            {
                id: "absolute-angled",
                format: "xStart (number), yStart (number), length (number)",
                func: () => `${random_location()} ${random_location()} ${random_size()}. `
            },
            {
                id: "absolute-x-y",
                format: "xStart (number), yStart (number), xMove (number), yMove (number)",
                func: () => `${random_location()} ${random_location()} ${random_size()} ${random_size()}. `
            }
        ]
    },
    "lis": {
        url: "List",
        name: "List",
        usage: [
            {
                id: "arguments",
                format: "[elem_1, ... elem_∞] (any)",
                func: () => `${random_list(1, 4).join(' ')} .`
            }
        ]
    },
    "lit": {
        url: "Lit",
        name: "Big Number",
        usage: [
            {
                id: "arguments",
                format: "[word_1, ... word_∞] (words)",
                func: () => `${random_var_name()} ${random_var_name()} . `
            }
        ]
    },
    "log": {
        url: "Log",
        name: "Log",
        usage: [
            {
                id: "base-e",
                format: "value (number)",
                func: () => `${15}.`
            },
            {
                id: "any-base",
                format: "base (number), value (number)",
                func: () => `${10} ${10 ** random_num_inclusive(1, 3)}.`
            }
        ]
    },
    "mod": {
        url: "Modulo",
        name: "Modulo",
        usage: [
            {
                id: "arguments",
                format: "n1 (number), n2 (number)",
                func: () => {
                    let val1 = random_num_inclusive(2, 5);
                    let val2 = random_num_inclusive(2, 5);
                    let result = random(1, val1);
                    return `${val1 * val2 + result} ${val1}. `
                }
            }
        ]
    },
    "mor": {
        url: "GreaterThan",
        name: "Greater Than",
        usage: [
            {
                id: "arguments",
                format: "n1 (number), n2 (number)",
                func: () => {
                    let num1 = random(0, 9);
                    let num2 = random(num1 + 1, 10);
                    return `${num1} ${num2}. `
                }
            }
        ]
    },
    "mov": {
        url: "MoveTo",
        name: "Move To",
        usage: [
            {
                id: "relative",
                format: "length (number)",
                func: () => `${random_size()}. `
            },
            {
                id: "absolute",
                format: "x (number), y (number)",
                func: () => `${random_location()} ${random_location()}. `
            }
        ]
    },
    "not": {
        url: "Ignore",
        name: "Ignore",
        usage: [
            {
                id: "arguments",
                format: "ignored (word)",
                func: () => `${random_var_name()}. `
            },
        ]
    },
    "oth": {
        url: "Or",
        name: "Or",
        usage: [
            {
                id: "arguments",
                format: "cond_1, cond_2, [cond3 ... cond_∞] (booleans or numbers)",
                func: () => `${random_bool()} ${random_bool()} . `
            }
        ]
    },
    "par": {
        url: "Comparison",
        name: "Comparison",
        usage: [
            {
                id: "arguments",
                format: "arg_1, arg_2, [arg_3, ... arg_∞] (any)",
                func: () => `${random_num_inclusive(2, 5)} ${random_num_inclusive(2, 5)} . `
            }
        ]
    },
    "pen": {
        url: "LineWidth",
        name: "Line Width",
        usage: [
            {
                id: "arguments",
                format: "size (number)",
                func: () => `${random_num_inclusive(2, 9)}. `
            }
        ]
    },
    "pol": {
        url: "Polygon",
        name: "Polygon",
        usage: [
            {
                id: "relative",
                format: "size (number), sides (number)",
                func: () => `${random_size()} ${random_sides()}. `
            },
            {
                id: "absolute",
                format: "x (number), y (number), size (number), sides (number)",
                func: () => `${random_location()} ${random_location()} ${random_size()} ${random_sides()}. `
            }
        ]
    },
    "pri": {
        url: "Print",
        name: "Print",
        usage: [
            {
                id: "single-word",
                format: "word (word)",
                func: () => `${random_var_name()}. `
            },
            {
                id: "default",
                format: "[x_1, ... x_∞] (any)",
                func: () => `"${random_var_name()}" ${random_math()} . `
            }
        ]
    },
    "ran": {
        url: "Random",
        name: "Random",
        usage: [
            {
                id: "random-float",
                format: "None",
                func: () => `. `
            },
            {
                id: "zero-to-bound",
                format: "upper_bound (number)",
                func: () => `${random_digit_neg()}. `
            },
            {
                id: "lower-to-upper-bound",
                format: "lower_bound (number), upper_bound (number)",
                func: () => `${random_digit_neg()} ${random_digit_neg()}. `
            },
        ]
    },
    "rec": {
        url: "Rectangle",
        name: "Rectangle",
        usage: [
            {
                id: "square-relative",
                format: "sideLength (number)",
                func: () => `${random_size()}. `
            },
            {
                id: "rectangle-relative",
                format: "width (number), height (number)",
                func: () => `${random_size()} ${random_size()}. `
            },
            {
                id: "square-absolute",
                format: "x (number), y (number), sideLength (number)",
                func: () => `${random_location()} ${random_location()} ${random_size()}. `
            },
            {
                id: "rectangle-absolute",
                format: "x (number), y (number), width (number), height (number)",
                func: () => `${random_location()} ${random_location()} ${random_size()} ${random_size()}. `
            }
        ]
    },
    "rep": {
        url: "Replace",
        name: "Replace",
        usage: [
            {
                id: "arguments",
                format: "list (list), index (number), value (any)",
                func: () => {
                    let list = random_list(2, 4);
                    let index = random(0, list.length);
                    let item = list[index];
                    list[index] = 0;
                    return `lis ${list.join(' ')} . ${index} ${item}. `
                }
            }
        ]
    },
    "ret": {
        url: "Return",
        name: "Return",
        usage: [
            {
                id: "return-nothing",
                format: "None",
                func: () => `. `
            },
            {
                id: "return-value",
                format: "value (any)",
                func: () => ` ${random_digit()}. `
            }
        ]
    },
    "roc": {
        url: "RoundedRectangle",
        name: "Rounded Rectangle",
        usage: [
            {
                id: "relative-square",
                format: "size (number)",
                func: () => `${random_size()}. `
            },
            {
                id: "relative-stretched",
                format: "width (number), height (number)",
                func: () => `${random_size()} ${random_size()}. `
            },
            {
                id: "absolute-square",
                format: "x (number), y (number), size (number)",
                func: () => `${random_location()} ${random_location()} ${random_size()}. `
            },
            {
                id: "absolute-stretched",
                format: "x (number), y (number), width (number), height (number)",
                func: () => `${random_location()} ${random_location()} ${random_size()} ${random_size()}. `
            }
        ]
    },
    "sin": {
        url: "Sine",
        name: "Sine",
        usage: [
            {
                id: "arguments",
                format: "value (number)",
                func: () => `${random_trig_angle()}. `
            }
        ]
    },
    "sta": {
        url: "Star",
        name: "Star",
        usage: [
            {
                id: "relative",
                format: "size (number)",
                func: () => `${random_size()}. `
            },
            {
                id: "absolute",
                format: "x (number), y (number), size (number)",
                func: () => `${random_location()} ${random_location()} ${random_size()}. `
            }
        ]
    },
    "sto": {
        url: "Stroke",
        name: "Stroke",
        usage: [
            {
                id: "color",
                format: "color (color)",
                func: () => `${random_color()}.`
            },
            {
                id: "rgb",
                format: "red (number), green (number), blue (number)",
                func: () => `${random_byte()} ${random_byte()} ${random_byte()}. `
            },
        ]
    },
    "sub": {
        url: "Subtract",
        name: "Subtract",
        usage: [
            {
                id: "one-argument-opposite",
                format: "num",
                func: () => `${random_digit()}. `
            },
            {
                id: "standard",
                format: "num_1 num_2",
                func: () => `${random_digit()} ${random_digit()}. `
            }
        ]
    },
    "tan": {
        url: "Tangent",
        name: "Tangent",
        usage: [
            {
                id: "arguments",
                format: "value (number)",
                func: () => `${random_trig_angle()}. `
            }
        ]
    },
    "tim": {
        url: "Multiply",
        name: "Multiply",
        usage: [
            {
                id: "arguments",
                format: "num_1 num_2 [num_3 ... num_∞].",
                func: () => `${random_digit()} ${random_digit()} . `
            }
        ]
    },
    "tri": {
        url: "Triangle",
        name: "Triangle",
        usage: [
            {
                id: "relative",
                format: "size (number)",
                func: () => `${random_size()}. `
            },
            {
                id: "absolute",
                format: "x (number), y (number), size (number)",
                func: () => `${random_location()} ${random_location()} ${random_size()}. `
            }
        ]
    },
    "tur": {
        url: "Rotate",
        name: "Rotate",
        usage: [
            {
                id: "arguments",
                format: "degrees (number)",
                func: () => `${random_angle()}. `
            }
        ]
    },
    "was": {
        url: "Variable",
        name: "Variable Assignement",
        usage: [
            {
                id: "arguments",
                format: "name (word), value (any)",
                func: () => `${random_var_name()} ${random_digit()}. `
            }
        ]
    },
    "whe": {
        url: "If",
        name: "If",
        usage: [
            {
                id: "Arguments",
                format: "condition (boolean or number), st1, [st_2, ... st_∞] (statements)",
                func: () => `${random_bool_equation()}\n[...]\n. `
            }
        ]
    },
    "whi": {
        url: "While",
        name: "While",
        usage: [
            {
                id: "arguments",
                format: "condition (any), [st_1, ... st_∞] (statements)",
                func: () => `${random_bool_equation()}\n[...]\n. `
            },
            {
                id: "special-case",
                format: "repeat (number), [st_1, ... st_∞] (statements)",
                func: () => `${random_digit()}\n[...]\n. `
            },
        ]
    },

}