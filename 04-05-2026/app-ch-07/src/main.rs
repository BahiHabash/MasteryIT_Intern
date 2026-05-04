use app_ch_07::hosting;

fn main() {
    hosting::add_to_waitlist();
    hosting::seat_at_table();
    
    println!("\n--- Running a full restaurant simulation ---");
    app_ch_07::eat_at_restaurant();
}
