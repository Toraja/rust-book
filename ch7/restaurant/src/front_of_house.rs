pub mod hosting;

pub mod serving {
    fn take_order() {
        println!("Are you ready to order?");
    }

    pub fn serve_order() {
        println!("Here you are. Enjoy your meal.");
    }

    fn take_payment() {
        println!("How would you like to pay?",);
    }
}
