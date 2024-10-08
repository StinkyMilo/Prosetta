let years_var = [[5, 8, 5, 3, 5, 6, 5], [3, 0, 6325, 4, 4, 3, 7], [2, 24344, 264830235, 43, 3643, 5, 4], [3, 35, 33, 6, 2, 8, 35], [7, `C`, 162632, 5, 2, 4, 200], [5, 5, 2, 100, 1, 50, 7], [8, 5, 0, 2000000000, 5, 100, 1000000000], [3, 9, 5, `slow`, `unreliable`, 3, 10], [30, 31, 32, 33, 34, 35, 36], [16, 12, 6, 6, 1, 10, 16]];
draw_rect(4333382, 38236);
set_fill("black");
set_stroke("white");
set_line_width(6);
rotate_delta(180);
let freedom_var = 20;
let bring_var = 10;
function new_var(state_var, digital_var, unity_var) {
    let machines_var = 90;
    let without_var = 0;
    move_to(digital_var, unity_var);
    while ((without_var < 7)) {
        if ((without_var < 4)) {
            rotate_delta(machines_var);
        }
        else {
            rotate_delta(-machines_var);
        }
        if ((years_var[state_var][without_var] > 4)) {
            draw_line(freedom_var);
        }
        else {
            move_to(freedom_var);
        }
        without_var = (without_var + 1);
    }
    rotate_delta(-machines_var);
}