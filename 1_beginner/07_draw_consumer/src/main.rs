use draw::{color, shapes::Rectangle};

fn main() {
    draw::draw_line(10, 23);
    let color = color::RGB { r: 247, g: 76, b: 0,};

    color::draw_line(20,10, &color);

    let square:Rectangle = draw::shapes::Rectangle {
        color,
        width: 32,
        height: 32,
    };

    println!("{square:?}");
}
