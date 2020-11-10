use crate::client::components::Position;

pub fn helper_in_rect(position: &Position, corner_1: &Position, corner_2: &Position) -> bool {
    let min_x = f32::min(corner_1.x, corner_2.x);
    let max_x = f32::max(corner_1.x, corner_2.x);
    let min_y = f32::min(corner_1.y, corner_2.y);
    let max_y = f32::max(corner_1.y, corner_2.y);

    if position.x >= min_x && position.x <= max_x && position.y >= min_y && position.y <= max_y {
        return true;
    }
    return false;
}

pub fn helper_rect_in_rect(r1: (&Position, &Position), r2: (&Position, &Position)) -> bool {
    let min_x = f32::min(r1.0.x, r1.1.x);
    let max_x = f32::max(r1.0.x, r1.1.x);
    let min_y = f32::min(r1.0.y, r1.1.y);
    let max_y = f32::max(r1.0.y, r1.1.y);

    let other_min_x = f32::min(r2.0.x, r2.1.x);
    let other_max_x = f32::max(r2.0.x, r2.1.x);
    let other_min_y = f32::min(r2.0.y, r2.1.y);
    let other_max_y = f32::max(r2.0.y, r2.1.y);

    let other_x_touch = min_x <= other_min_x && other_min_x <= max_x;
    let other_y_touch = min_y <= other_min_y && other_min_y <= max_y;
    let x_touch = other_min_x <= min_x && min_x <= other_max_x;
    let y_touch = other_min_y <= min_y && min_y <= other_max_y;
    if other_x_touch && other_y_touch {
        return true;
    }
    if x_touch && y_touch {
        return true;
    }
    if x_touch && other_y_touch {
        return true;
    }
    if other_x_touch && y_touch {
        return true;
    }
    return false;
}
