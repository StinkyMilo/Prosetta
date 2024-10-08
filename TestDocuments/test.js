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
