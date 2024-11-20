use crate::line_item::LineItem;
use crate::sale::Sale;
use chrono::Utc;
use colored::Colorize;
use nanoid::nanoid;
use std::io::{self, Write};

pub struct Pos {
    pub products: Vec<LineItem>,
    pub sales: Vec<Sale>,
}

impl Pos {
    pub fn new() -> Pos {
        Pos { products: Vec::new(), sales: Vec::new() }
    }

    pub fn add_product(&mut self, product: LineItem) {
        self.products.push(product);
        println!("{}", "Product added!".bold().green());
    }

    pub fn view_cart(&self) {
        if self.products.is_empty() {
            println!("{}", "The cart is empty".bold().yellow());
            return;
        }

        println!("{}", "--------------- CART ---------------".yellow());
        for product in &self.products {
            println!(
                "${:.2} x {}",
                product.price, product.quantity
            );
        }
        println!("{}", "------------------------------------".yellow());
    }

    pub fn checkout(&mut self) {
        if self.products.is_empty() {
            println!("{}", "The cart is empty".bold().yellow());
            return;
        }

        let total: f32 = self.products.iter().map(|p| p.price * p.quantity as f32).sum();

        let sale = Sale::new(Utc::now(), total, nanoid!());
        let cloned_sale = sale.clone();
        self.sales.push(cloned_sale);

        println!("{}", "----------- FINALIZED SALE -----------".green());
        println!("Total: ${}", total);
        println!("Date and Time: {}", sale.clone().date.format("%d-%m-%Y %H:%M:%S"));
        println!("Transaction ID: ${}", sale.clone().id);
        println!("{}", "--------------------------------------".green());

        self.products.clear();
    }

    pub fn view_sales(&self) {
        if self.sales.is_empty() {
            println!("{}", "There are no sales to display".bold().yellow());
            return;
        }
        println!("{}", "---------------- SALES ---------------".green());
        for sale in &self.sales {
            println!("--------------------------------------");
            println!("Total: ${}", sale.total);
            println!("Date and Time: {}", sale.clone().date.format("%d-%m-%Y %H:%M:%S"));
            println!("Transaction ID: ${}", sale.id);
            println!("--------------------------------------");
        }
        println!("{}", "--------------------------------------".green());
    }

    pub fn get_input(prompt: &str) -> String {
        print!("{}", prompt);
        io::stdout().flush()
            .expect(&format!("{}", "Error ::: Unable to flush stdout".bold().red()));
        let mut input = String::new();
        io::stdin()
            .read_line(&mut input)
            .expect(&format!("{}", "Failed to read input".bold().red()));
        input.trim().to_string()
    }
}
