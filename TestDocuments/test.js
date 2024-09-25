let arraymario = [10, 30, 2, 15, 19, 24, 60, 80, 0];
function middlemario(startmario, endmario) {
    let lenmario = (endmario - startmario);
    if ((lenmario % 2) == 0) {
        return (startmario + (lenmario / 2));
    }
    return (startmario + ((lenmario - 1) / 2));
}
function mergemario(arraymario, startmario, endmario) {
    let differencemario = (endmario - startmario);
    if ((differencemario < 1)) {
        return [];
    }
    if (differencemario == 1) {
        return [arraymario[startmario]];
    }
    let midmario = middlemario(startmario, endmario);
    let leftmario = mergemario(arraymario, startmario, midmario);
    let rightmario = mergemario(arraymario, midmario, endmario);
    let outputmario = [];
    let yinmario = 0;
    let yangmario = 0;
    while (((yinmario < leftmario.length) || (yangmario < rightmario.length))) {
        if ((((yinmario + 1) > leftmario.length) || (leftmario[yinmario] > rightmario[yangmario]))) {
            outputmario.push(rightmario[yangmario]);
            yangmario = (yangmario + 1);
        }
        else {
            outputmario.push(leftmario[yinmario]);
            yinmario = (yinmario + 1);
        }
        return outputmario;
    }
}
let valuemario = mergemario(arraymario, 0, arraymario.length);
console.log(valuemario);