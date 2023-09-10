#![allow(dead_code)]
#![allow(unused_variables)]

trait Vehicle: Paint{
    fn park(&self);
    fn get_default_color()-> String {
        "black".to_owned()
    }
}

trait Paint{
    fn paint(&self, color: String){
        println!("Painting Object: {}", color);
    }
}

#[derive(Debug, PartialEq)]
struct VehicleInfo{
    make: String,
    model: String,
    year: u16
}

#[derive(Debug, PartialEq)]
struct Car{
    info: VehicleInfo,
}

#[derive(Debug, PartialEq)]
struct Truck{
    info: VehicleInfo,
}

impl Vehicle for Car{
    fn park(&self){
        println!("Parking Car!");
    }
}

impl Paint for Car{}
impl Paint for Truck{}

impl Truck {
    fn unload(&self){
        println!("Unloading Truck.");
    }
}

struct House{}

impl Paint for House{
    fn paint(&self, color: String){
        println!("Painting House: {}", color);
    }
}

impl Vehicle for Truck{
    fn park(&self){
        println!("Parking Truck!");
    }
}

fn main() {
    let car = Car{
        info: VehicleInfo { make: "Honda".to_owned(), model: "Civic".to_owned(), year: 1995 }
    };

    let car2 = Car{
        info: VehicleInfo { make: "Tesla".to_owned(), model: "ModelY".to_owned(), year: 2020 }
    };

    let truck = Truck{
        info: VehicleInfo { make: "Honda".to_owned(), model: "RidgeLine".to_owned(), year: 2022 }
    };
    let house = House{};
    let object = create_paintable_object(true);

    let paintable_objects: Vec<&dyn Paint> = vec![&car, &house];

    paint_red(&truck);
    paint_red(&house);
    paint_red(object.as_ref());

    println!("{}", car == car2);
}

// fn paint_red<T: Paint>(object: T) {
fn paint_red(object: &dyn Paint) {
    object.paint("Red".to_owned());
}

fn create_paintable_object(vehicle: bool) -> Box<dyn Paint>{

    if vehicle{
        Box::new(Car{ 
            info: VehicleInfo { make: "Honda".to_owned(), model: "Civic".to_owned(), year: 1995 }
        })
    } else {
        Box::new(House{})
    }
}