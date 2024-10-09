function all_var(num_var, x_var, y_var) {
    let temp_var = (num_var / 10);
    let num1_var = (temp_var - (temp_var % 1));
    let num2_var = (num_var % 10);
    drawsingledigitnum_var(num1_var, x_var, y_var)
    drawsingledigitnum_var(num2_var, (x_var + segdist_var + padding_var), y_var)
}

function drawnum_var(num_var, x_var, y_var) {
    let temp_var = (num_var / 10);
    let num1_var = (temp_var - (temp_var % 1));
    let num2_var = (num_var % 10);
    drawsingledigitnum_var(num1_var, x_var, y_var)
    drawsingledigitnum_var(num2_var, (x_var + segdist_var + padding_var), y_var)
}