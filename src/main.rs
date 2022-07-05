// use model::{BookEntry, EntryType};
mod commands;
mod db;
mod model;
mod ui;
fn main() {
    ui::draw().expect("Error starting UI");
}
