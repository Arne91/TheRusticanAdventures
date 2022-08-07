use std::{fs::File,io, io::Read};

//https://rust-by-example-ext.com/ndarray.html
use ndarray::Array2;

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
    let array_2d = Array2::<u8>::zeros((10,10));

    println!("{array_2d}");


    let _result = load_map("./src/maps/first_map.map");
    //let _result = load_map("/home/arne/Workspace/Rust/TheRusticanAdventures/src/maps/first_map.map");
    let pos:PositionFigure = PositionFigure::create(String::from("Wald"), String::from("Steppe"), String::from("Mauer"), String::from("Mauer"));
    show_map(pos);
}

fn load_map(map_file: &str)->Result<(), io::Error>{
    println!("Load Map: {map_file}");

    let _result = match File::open(map_file){
        Ok(mut f) => {
            let mut content = String::new();
            f.read_to_string(&mut content)?;

            let content_vec1: Vec<_> = content.split("\n").collect();
            for elem in content_vec1.iter(){
                let content_vec2: Vec<_> = elem.split(";").collect();
                println!("{:?}", content_vec2);

            }
        },
        Err(_) => println!("Error while reading!")
    };

    Ok(())

    // TODO the function should load the map file and store it in a variable.
}

// TODO show_map should be a function of a map-variable
fn show_map(pos: PositionFigure){
    println!("Vor dir befindet sich {}", pos.front);
    println!("Rechts von dir befindet sich {}", pos.right);
    println!("Links von dir befindet sich {}", pos.left);
    println!("Hinter dir befindet sich {}", pos.back);
}