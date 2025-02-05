/**
 * @param takes list of functions or values
 * @return {string}
 */
function format(...args) {
    let ret = "";
    for (let arg of args) {
        if (arg instanceof Function) {
            ret += " " + arg();
        } else {
            ret += " " + arg;
        }
    }
    return ret;
}

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

function random_num_inclusive(min, max) {
    return random(min, max, 1)
}

// /**
//  * @param copies 
//  */
// function copy_between(min, max, arg) {
//     let copy_num = random(min, max + 1);
//     return copy_count(copy_num, arg);
// }

/**
 * @return {number} between min and max (inclusive)
 */
function random_digit() {
    return random(0, 10)
}

/**
 * @return {number} between min and max (inclusive)
 */
function random_digit_neg() {
    return random(-9, 9, 1)
}

function random_byte() {
    return random(255)
}

/**
 * @return {number} between min and max (inclusive)
 */
function random_angle() {
    return random(15, 90, 15)
}

/**
 * @return {number} between min and max (inclusive)
 */
function random_trig_angle() {
    return random(-90, 270, 15)
}

/**
 * @return {number} between min and max (inclusive)
 */
function random_size() {
    return random(10, 90, 10)
}

/**
 * @return {number} between min and max (inclusive)
 */
function random_location() {
    return random(-75, 75, 25)
}

/**
 * @return {number} between min and max (inclusive)
 */
function random_list(min_start, length = 3) {
    let start = random_num_inclusive(min_start, 10 - length);
    let arr = [...new Array(length).keys()].map((_, i) => i + start)
    return arr
}


/**
 * @return {string} color
 */
function random_color() {
    let colors = ["blue", "cyan", "gold", "gray", "grey", "lime", "navy", "pink", "plum", "snow", "teal"];
    return colors[random_num_inclusive(0, 5)];
}

/**
 * @return {string} color
 */
function random_bool() {
    return colors[random_num_inclusive(0, 1)];
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
    "add": {
        url: "Add",
        name: "Add",
        usage: [
            {
                id: "arguments",
                format: "num_1 num_2 [num_3 ... num_∞].",
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
                func: () => `${"TODO"}. `
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
                func: () => `lis ${random_list(2, random_num_inclusive(2, 3)).join(" ")} . . `
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
                func: () => `${"TODO"}. `
            },
            {
                id: "specified-location",
                format: "list (list), index (number)",
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
            }
        ]
    },
    "exp": {
        url: "Exponentiate",
        name: "Exponentiate",
        usage: [
            {
                id: "standard",
                format: "base (number), exp (number)",
                func: () => `${"TODO"}. `
            },
            {
                id: "ex",
                format: "exp (number)",
                func: () => `${"TODO"}. `
            },
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
                func: () => `${"TODO"}. `
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
        usage: []
    },
    "fun": {
        url: "Function",
        name: "Function",
        usage: [
            {
                id: "arguments",
                format: "[arg_1, ... arg_∞] (words) [CLOSING PUNCTUATION], [st_1, ... st_∞] (statements)",
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
            },
            {
                id: "relative-stretched",
                format: "width (number), height (number)",
                func: () => `${"TODO"}. `
            },
            {
                id: "absolute-regular",
                format: "x (number), y (number), size (number)",
                func: () => `${"TODO"}. `
            },
            {
                id: "absolute-stretched",
                format: "x (number), y (number), width (number), height (number)",
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
            },
            {
                id: "relative-stretched",
                format: "width (number), height (number)",
                func: () => `${"TODO"}. `
            },
            {
                id: "absolute-regular",
                format: "x (number), y (number), size (number)",
                func: () => `${"TODO"}. `
            },
            {
                id: "absolute-stretched",
                format: "x (number), y (number), width (number), height (number)",
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
            },
            {
                id: "relative-x-y",
                format: "xMove (number), yMove (number)",
                func: () => `${"TODO"}. `
            },
            {
                id: "absolute-angled",
                format: "xStart (number), yStart (number), length (number)",
                func: () => `${"TODO"}. `
            },
            {
                id: "absolute-x-y",
                format: "xStart (number), yStart (number), xMove (number), yMove (number)",
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
            }
        ]
    },
    "mov": {
        url: "MoveTo",
        name: "Move To",
        usage: [
            {
                id: "absolute",
                format: "x (number), y (number)",
                func: () => `${"TODO"}. `
            },
            {
                id: "relative",
                format: "length (number)",
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
            },
            {
                id: "absolute",
                format: "x (number), y (number), size (number), sides (number)",
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
            },
            {
                id: "default",
                format: "```[x_1, ... x_∞] (any)`",
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
            },
            {
                id: "square-absolute",
                format: "x (number), y (number), sideLength (number)",
                func: () => `${"TODO"}. `
            },
            {
                id: "rectangle-relative",
                format: "width (number), height (number)",
                func: () => `${"TODO"}. `
            },
            {
                id: "rectangle-absolute",
                format: "x (number), y (number), width (number), height (number)",
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
            },
            {
                id: "return-value",
                format: "value (any)",
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
            },
            {
                id: "relative-stretched",
                format: "width (number), height (number)",
                func: () => `${"TODO"}. `
            },
            {
                id: "absolute-square",
                format: "x (number), y (number), size (number)",
                func: () => `${"TODO"}. `
            },
            {
                id: "absolute-stretched",
                format: "x (number), y (number), width (number), height (number)",
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
            },
            {
                id: "absolute",
                format: "x (number), y (number), size (number)",
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
            },
            {
                id: "absolute",
                format: "x (number), y (number), size (number)",
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
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
                func: () => `${"TODO"}. `
            },
            {
                id: "special-case",
                format: "repeat (number), [st_1, ... st_∞] (statements)",
                func: () => `${"TODO"}. `
            },
        ]
    },

}