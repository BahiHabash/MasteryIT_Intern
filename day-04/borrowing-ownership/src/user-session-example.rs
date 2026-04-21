struct UserSession {
    user_id: u32,
    username: String,
    role: String,
    last_active: u64,
}

// 1. IMMUTABLE BORROW: Checking permissions (Needs to read, not modify)
fn is_admin(session: &UserSession) -> bool {
    session.role == "admin"
}

// 2. MUTABLE BORROW: Updating state
fn heartbeat(session: &mut UserSession, current_time: u64) {
    session.last_active = current_time;
}

// 3. TAKING OWNERSHIP: Destructive operations / Data serialization
fn save_to_database_and_destroy(session: UserSession) {
    // The database driver consumes the struct.
    println!("Saving {} to DB...", session.username);
    // `session` is dropped (deleted from memory) at the end of this function.
}

fn handle_http_request() {
    let mut current_session = UserSession {
        user_id: 101,
        username: String::from("alice_dev"),
        role: String::from("admin"),
        last_active: 100000,
    };

    // We can read multiple times
    if is_admin(&current_session) {
        println!("Access granted.");
    }

    // We borrow mutably to update the heartbeat
    heartbeat(&mut current_session, 100050);

    // Finally, the request is over, we hand over ownership to the DB layer
    save_to_database_and_destroy(current_session);

    // println!("{}", current_session.username); // ERROR: Session no longer exists here.
}