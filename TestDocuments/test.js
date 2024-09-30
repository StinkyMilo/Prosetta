let array_var = [10, 30, 2, 15, 19, 24, 60, 80, 0];
function middle_var(start_var, end_var) {
    let len_var = (end_var - start_var);
    if ((len_var % 2) == 0) {
        return (start_var + (len_var / 2));
    }
    return (start_var + ((len_var - 1) / 2));
}
function merge_var(array_var, start_var, end_var) {
    let difference_var = (end_var - start_var);
    if ((difference_var < 1)) {
        return [];
    }
    if (difference_var == 1) {
        return [array_var[start_var]];
    }
    let mid_var = middle_var(start_var, end_var);
    let left_var = merge_var(array_var, start_var, mid_var);
    let right_var = merge_var(array_var, mid_var, end_var);
    let output_var = [];
    let yin_var = 0;
    let yang_var = 0;
    while (((yin_var < left_var.length) || (yang_var < right_var.length))) {
        if ((((yin_var + 1) > left_var.length) || (left_var[yin_var] > right_var[yang_var]))) {
            output_var.push(right_var[yang_var]);
            yang_var = (yang_var + 1);
        }
        else {
            output_var.push(left_var[yin_var]);
            yin_var = (yin_var + 1);
        }
        return output_var;
    }
}
let value_var = merge_var(array_var, 0, array_var.length);
console.log(value_var);
