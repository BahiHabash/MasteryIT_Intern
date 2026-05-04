mod back_office;
pub mod front_of_house;
pub use crate::front_of_house::hosting;

pub fn eat_at_restaurant() {    
    crate::front_of_house::hosting::add_to_waitlist();

    front_of_house::hosting::seat_at_table();
    back_office::fix_incorrect_order();
}

fn deliver_order() {
    println!("Order delivered to the customer!");
}
