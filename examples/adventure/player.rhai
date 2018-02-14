let walk_speed = 250.0;
let animation_speed = 12.0;

fn calculate_animation_row(row, hor, ver) {    
    if ver == -1.0 {
        return 2.0
    } 
    
    if ver == 1.0 {
        return 0.0
    }

    if hor == -1.0 {
        return 3.0
    } 
    
    if hor == 1.0 {
        return 1.0
    } 

    row
}

let column = animation_column;

if horizontal_direction != 0.0 || vertical_direction != 0.0 {
    column = animation_column + delta * animation_speed;
}

player.animation_step(calculate_animation_row(animation_row, horizontal_direction, vertical_direction), column);
player.mov(delta * walk_speed * horizontal_direction, delta * walk_speed * vertical_direction, map);
player