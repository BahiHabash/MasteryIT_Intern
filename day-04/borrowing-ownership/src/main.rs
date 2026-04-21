fn main() {
    // create a MUTABLE variable.
    let mut server_status = String::from("Online");

    // pass it as an IMMUTABLE reference (&).
    print_status(&server_status);

    // pass it as a MUTABLE reference (&mut).
    update_status(&mut server_status);

    // MOVE it (transfer ownership).
    consume_and_destroy(server_status);

    /*
        let mut name: String = String::from("Bahi");

        println!("{name}");

        let name_imm = &name;
        let name_mut = &mut name;  // Error: we can't have mutable ref while there is immutable reference in use

        println!("{name_imm}");    // because of this line (name_imm) still in use and prevent the creation of mutable ref until no longer used
        println!("{name_mut}");
    */
}

// Accepts an immutable reference. Cannot edit.
fn print_status(status: &String) {
    println!("Current status is: {}", status);
}

// Accepts a mutable reference. Can edit.
fn update_status(status: &mut String) {
    status.push_str(" - Under Heavy Load");
}

// Accepts by value (Moves ownership).
fn consume_and_destroy(status: String) {
    println!("Logging final state to DB before shutting down: {}", status);
    // `status` is dropped (deleted) from memory right here.
}