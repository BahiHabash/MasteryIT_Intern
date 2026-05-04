pub fn fix_incorrect_order() {
    println!("Kitchen: Fixing an incorrect order...");
    cook_order();
    
    super::deliver_order();
}

fn cook_order() {
    println!("Kitchen: Cooking food.");
}
