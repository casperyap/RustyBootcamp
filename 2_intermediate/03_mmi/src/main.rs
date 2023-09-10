#![allow(dead_code)]
#![allow(unused_variables)]

trait UIComponent{
    fn render(&self){
        println!("Rendering component...");
    }
}

struct Button{
    text: String,
}

impl UIComponent for Button{}

//3. Use Box for recursive type to fix the memory size.
struct Container{
    name: String,
    child: Box<Container>,
}

fn main(){
    let button_a = Button{text: "Button A".to_owned()};
    let button_b = Box::new(Button{text: "Button B".to_owned()});

    let button_c = button_a;
    let button_d = button_b; //1. Faster as only the box is copied instead the whole struct.

    //2. Use Box to store containers of objects with same triats.
    let components: Vec<Box<dyn UIComponent>> = vec![
        Box::new(button_c),
        button_d
    ];
}