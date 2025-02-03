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

function random(min, max) {
    return Math.floor(Math.random() * (max - min) + min);
}

// /**
//  * @param copies 
//  */
// function copy_between(min, max, arg) {
//     let copy_num = random(min, max + 1);
//     return copy_count(copy_num, arg);
// }

/**
 * @param copies 
 */
function copy_count(num, arg) {
    return Array(num).fill(arg);
}

/**
 * @return {number} between min and max (inclusive)
 */
function random_digit() {
    return random(0, 10)
}

/**
 * @return {number} between min and max (inclusive)
 */
function random_angle() {
    return random(1, 4) * 15
}

/**
 * @return {string} color
 */
function random_color() {
    let colors = ["blue", "cyan", "gold", "gray", "grey", "lime", "navy", "pink", "plum", "snow", "teal"];
    return () => colors[random_num_inclusive(0, 5)];
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
        usage: []
    },
    "app": {
        url: "Append",
        name: "Append",
        usage: []
    },
    "arc": {
        url: "Ellipse",
        name: "Ellipse",
        usage: []
    },
    "bez": {
        url: "Bezier",
        name: "Bezier",
        usage: []
    },
    "col": {
        url: "Color",
        name: "Color",
        usage: []
    },
    "cos": {
        url: "Cosine",
        name: "Cosine",
        usage: []
    },
    "cou": {
        url: "Length",
        name: "Length",
        usage: []
    },
    "del": {
        url: "Delete",
        name: "Delete",
        usage: []
    },
    "els": {
        url: "Else",
        name: "Else",
        usage: []
    },
    "exp": {
        url: "Exponentiate",
        name: "Exponentiate",
        usage: []
    },
    "fil": {
        url: "Fill",
        name: "Fill",
        usage: []
    },
    "fin": {
        url: "Find",
        name: "Find",
        usage: []
    },
    "flo": {
        url: "Floor",
        name: "Floor",
        usage: []
    },
    "fra": {
        url: "Frame",
        name: "Frame",
        usage: []
    },
    "fre": {
        url: "Foreach",
        name: "For Each",
        usage: []
    },
    "fun": {
        url: "Function",
        name: "Function",
        usage: []
    },
    "hea": {
        url: "Heart",
        name: "Heart",
        usage: []
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
        usage: []
    },
    "int": {
        url: "Int",
        name: "Small Number",
        usage: []
    },
    "inv": {
        url: "Not",
        name: "Not",
        usage: []
    },
    "kir": {
        url: "Kirby",
        name: "Kirby",
        usage: []
    },
    "les": {
        url: "LessThan",
        name: "Less Than",
        usage: []
    },
    "lin": {
        url: "Line",
        name: "Line",
        usage: []
    },
    "lis": {
        url: "List",
        name: "List",
        usage: []
    },
    "lit": {
        url: "Lit",
        name: "Big Number",
        usage: []
    },
    "log": {
        url: "Log",
        name: "Log",
        usage: []
    },
    "mod": {
        url: "Modulo",
        name: "Modulo",
        usage: []
    },
    "mor": {
        url: "GreaterThan",
        name: "Greater Than",
        usage: []
    },
    "mov": {
        url: "MoveTo",
        name: "Move To",
        usage: []
    },
    "not": {
        url: "Ignore",
        name: "Ignore",
        usage: []
    },
    "oth": {
        url: "Or",
        name: "Or",
        usage: []
    },
    "par": {
        url: "Comparison",
        name: "Comparison",
        usage: []
    },
    "pen": {
        url: "LineWidth",
        name: "Line Width",
        usage: []
    },
    "pol": {
        url: "Polygon",
        name: "Polygon",
        usage: []
    },
    "pri": {
        url: "Print",
        name: "Print",
        usage: []
    },
    "ran": {
        url: "Random",
        name: "Random",
        usage: []
    },
    "rec": {
        url: "Rectangle",
        name: "Rectangle",
        usage: []
    },
    "rep": {
        url: "Replace",
        name: "Replace",
        usage: []
    },
    "ret": {
        url: "Return",
        name: "Return",
        usage: []
    },
    "roc": {
        url: "RoundedRectangle",
        name: "Rounded Rectangle",
        usage: []
    },
    "sin": {
        url: "Sine",
        name: "Sine",
        usage: []
    },
    "sta": {
        url: "Star",
        name: "Star",
        usage: []
    },
    "sto": {
        url: "Stroke",
        name: "Stroke",
        usage: []
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
        usage: []
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
        usage: []
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
        name: "Variable Assigneme",
        usage: []
    },
    "whe": {
        url: "If",
        name: "If",
        usage: []
    },
    "whi": {
        url: "While",
        name: "While",
        usage: []
    },

}