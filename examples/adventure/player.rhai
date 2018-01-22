let walk_speed = 250.0;

fn calculate_animation_step(step, hor, ver) {
    if hor == -1.0 {
        if step >= 12.0 && step <= 15.0 {
            if step < 15.0 {
                return step + 1.0
            } else {
                return 12.0
            }
        } else {
            return 12.0
        }
    }

    if hor == 1.0 {
        if step >= 4.0 && step <= 7.0 {
            if step < 7.0 {
                return step + 1.0
            } else {
                return 4.0
            }
        } else {
            return 4.0
        }
    }

    if ver == -1.0 {
        if step >= 8.0 && step <= 11.0 {
            if step < 11.0 {
                return step + 1.0
            } else {
                return 8.0
            }
        } else {
            return 8.0
        }
    }

    if ver == 1.0 {
        if step >= 0.0 && step <= 3.0 {
            if step < 3.0 {
                return step + 1.0
            } else {
                return 0.0
            }
        } else {
            return 0.0
        }
    }

    step
}

player.animation_step(calculate_animation_step(animation_step, horizontal_direction, vertical_direction));
player.mov(delta * walk_speed * horizontal_direction, delta * walk_speed * vertical_direction);
player