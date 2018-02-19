let walk_speed = 250.0;
let animation_speed = 12.0;

let hor = 1.0;

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

if hor != 0.0 {
    column = animation_column + delta * animation_speed;
}

//npc.animation_step(calculate_animation_row(animation_row, hor, 0.0), column);
//npc.mov(delta * walk_speed * hor, 0.0, map);
npc