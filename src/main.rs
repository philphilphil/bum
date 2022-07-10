// use model::{BookEntry, EntryType};
mod calculation;
mod commands;
mod db;
mod model;
mod ui;
fn main() {
    db::ensure_db_files_exist().unwrap();
    ui::draw().expect("Error starting UI");
}
