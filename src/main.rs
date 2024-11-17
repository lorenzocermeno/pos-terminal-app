extern crate colored;
mod pos;
mod product;
mod sale;
use colored::*;

fn main() {
    let mut pos = pos::Pos::new();

    loop {
        println!("Choose an option ...\n\
              1. Initialize sale\n\
              2. View cart\n\
              3. Finalize sale\n\
              4. View sales\n\
              5. Exit");

        let choice = pos::Pos::get_input("Option: ");
        match choice.as_str() {
            "1" => {
                let price: f32 = pos::Pos::get_input("Enter product price: ")
                    .parse()
                    .expect(&format!("{}", "Invalid price".bold().red()));
                let quantity: u32 = pos::Pos::get_input("Enter product quantity: ")
                    .parse()
                    .expect(&format!("{}", "Invalid quantity".bold().red()));
                let product = product::Product::new(price, quantity);
                pos.add_product(product);
            }
            "2" => {
                pos.view_cart();
            }
            "3" => {
                pos.checkout();
            }
            "4" => {
                pos.view_sales();
            }
            "5" => {
                println!("{}", "Exiting ...".bold().red());
                break;
            }
            _ => {
                println!("{}", "Invalid option. Please try again.".bold().red());
            }
        }
    }
}