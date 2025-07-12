pub enum Zone {
    Square {x:i32,y:i32,r:i32},
    // (x,y) center and r radius (semi-side)
    Rectangle {x:i32,y:i32,r_h:i32,r_v:i32},
    // (x,y) center, r_h horizontal radius, r_v vertical radius
    Circle {x:i32,y:i32,r:i32}
}

pub struct Canvas {
    size:u32,
    zones:Vec<Zone>,
    player:Player
}
