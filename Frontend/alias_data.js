/**
 * @param takes list of functions or values
 * @return {string}
 */
function format(...args) {
    let ret = "";
    for (let arg in args) {
        if (arg instanceof Function) {
            ret += arg();
        } else {
            ret += arg;
        }
    }
    return ret;
}

/**
 * @param copies 
 */
function copy_between(min, max, arg) {
    let copy_num = random_num_inclusive(min, max);
    return copy_count(copy_num, arg);
}

/**
 * @param copies 
 */
function copy_count(num, arg) {
    return Array(num).fill(arg);
}

/**
 * @return {number} between min and max (inclusive)
 */
function random_num_inclusive(min, max) {
    return () => Math.floor(Math.random() * (max - min + 1) + min);
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
        usage: [
            {
                id: "arguments",
                format: "num_1 num_2 [num_3 ... num_∞]",
                func: () => format(...copy_between(2, 4, random_num_inclusive(1, 10)), ".")
            }
        ]
    },
    "als": {
        url: "And",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "app": {
        url: "Append",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "arc": {
        url: "Ellipse",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "bez": {
        url: "Bezier",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "col": {
        url: "Color",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "cos": {
        url: "Cosine",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "cou": {
        url: "Length",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "del": {
        url: "Delete",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "els": {
        url: "Else",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "exp": {
        url: "Exponentiate",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "fil": {
        url: "Fill",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "fin": {
        url: "Find",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "flo": {
        url: "Floor",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "fra": {
        url: "Frame",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "fre": {
        url: "Foreach",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "fun": {
        url: "Function",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "hea": {
        url: "Heart",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "ide": {
        url: "Divide",
        usage: [
            {
                id: "arguments",
                format: "num_1 num_2",
                func: () => {
                    let val1 = random_num_inclusive(2, 5);
                    let val2 = random_num_inclusive(2, 5);
                    format(val1 * val2, val1, ".")
                }
            }
        ]
    },
    "ind": {
        url: "Index",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "int": {
        url: "Int",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "inv": {
        url: "Not",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "kir": {
        url: "Kirby",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "les": {
        url: "LessThan",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "lin": {
        url: "Line",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "lis": {
        url: "List",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "lit": {
        url: "Lit",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "log": {
        url: "Log",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "mod": {
        url: "Modulo",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "mor": {
        url: "GreaterThan",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "mov": {
        url: "MoveTo",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "not": {
        url: "Ignore",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "oth": {
        url: "Or",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "par": {
        url: "Comparison",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "pen": {
        url: "LineWidth",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "pol": {
        url: "Polygon",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "pri": {
        url: "Print",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "ran": {
        url: "Random",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "rec": {
        url: "Rectangle",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "rep": {
        url: "Replace",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "ret": {
        url: "Return",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "roc": {
        url: "RoundedRectangle",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "sin": {
        url: "Sine",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "sta": {
        url: "Star",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "sto": {
        url: "Stroke",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "sub": {
        url: "Subtract",
        usage: [
            {
                id: "one-argument-opposite",
                format: "num",
                func: () => format(random_num_inclusive(1, 10), ".")
            },
            {
                id: "standard",
                format: "num_1 num_2",
                func: () => format(...copy_count(2, random_num_inclusive(1, 10)), ".")
            }
        ]
    },
    "tan": {
        url: "Tangent",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "tim": {
        url: "Multiply",
        usage: [
            {
                id: "arguments",
                format: "num_1 num_2 [num_3 ... num_∞]",
                func: () => format(...copy_between(2, 4, random_num_inclusive(2, 5)), ".")
            }
        ]
    },
    "tri": {
        url: "Triangle",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "tur": {
        url: "Rotate",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "was": {
        url: "Variable",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "whe": {
        url: "If",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },
    "whi": {
        url: "While",
        usage: [
            {
                format: "FORMAT",
                func: () => "f"
            }
        ]
    },

}