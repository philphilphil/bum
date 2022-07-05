// use model::{BookEntry, EntryType};
mod commands;
mod db;
mod model;
mod ui;
fn main() {
    // db::create_test_bookings();
    // db::get_bookings();
    // let exp = BookEntry::new("eins", EntryType::Income, 5, 13.22);
    // db::add_booking(exp);
    // db::get_bookings();

    ui::draw().expect("Error starting UI");
}
