struct PositionFigure{
    front: String,
    right: String,
    left: String,
    back: String
}

impl PositionFigure{
    fn create (front: String, right: String, left: String, back: String)->PositionFigure{
        PositionFigure{front,right,left, back }
    }
}


fn main() {
    load_map("maps/first_map.map");
    let pos:PositionFigure = PositionFigure::create(String::from("Wald"), String::from("Steppe"), String::from("Mauer"), String::from("Mauer"));
    show_map(pos);
}

fn load_map(map_file: &str){
    println!("{map_file}");
    // TODO the function should load the map file and store it in a variable.
}

// TODO show_map should be a function of a map-variable
fn show_map(pos: PositionFigure){
    println!("Vor dir befindet sich {}", pos.front);
    println!("Rechts von dir befindet sich {}", pos.right);
    println!("Links von dir befindet sich {}", pos.left);
    println!("Hinter dir befindet sich {}", pos.back);
}