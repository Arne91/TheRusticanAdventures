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
    let pos:PositionFigure = PositionFigure::create(String::from("Wald"), String::from("Steppe"), String::from("Mauer"), String::from("Mauer"));
    show_map(pos);
}


fn show_map(pos: PositionFigure){
    println!("Vor dir befindet sich {}", pos.front);
    println!("Rechts von dir befindet sich {}", pos.right);
    println!("Links von dir befindet sich {}", pos.left);
    println!("Hinter dir befindet sich {}", pos.back);
}