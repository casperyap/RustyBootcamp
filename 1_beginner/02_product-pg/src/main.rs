enum ProductCategory {
    Books,
    Clothings,
    Electronics,
}

struct Product{
    name: String,
    category: ProductCategory,
    price: f32,
    in_stock: bool,
}

impl Product{
    fn new(name: String, category: ProductCategory, price: f32) -> Product{
        Product{
            name: name,
            category: ProductCategory::Electronics,
            price: price,
            in_stock: true,
        }
    }
    fn get_default_sales_tax() -> f32{
        0.1
    }
    
    fn calculate_sales_tax(&self) -> f32{
        self.price * 0.1
    }

    fn set_price(&mut self, price: f32){
        self.price = price;
    }

    fn buy(self) -> i32{ //self will die after this function!
        let name: String = self.name;
        println!("{name} was bought!");
        123
    }
}


fn main() {
    let mut book = Product::new(
        String::from("Book"), ProductCategory::Electronics, 28.85
     
    );

    let sales_tax = book.calculate_sales_tax();
    println!("Sales tax: {}", sales_tax);
    book.set_price(1.0);
    book.buy();
}
