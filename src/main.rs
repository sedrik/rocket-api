#![feature(plugin)]
#![plugin(rocket_codegen)]
extern crate rocket;
#[macro_use]
extern crate diesel;
#[macro_use]
extern crate diesel_codegen;
extern crate dotenv;
extern crate serde_json;
#[macro_use]
extern crate lazy_static;
#[macro_use]
extern crate rocket_contrib;
#[macro_use]
extern crate serde_derive;
extern crate r2d2;
extern crate r2d2_diesel;
extern crate uuid;

mod schema;
mod db;
mod note;
mod models;

use db::DB;
use note::{get_notes, get_note, create_note, delete_note, update_note};
use models::*;
use rocket_contrib::JSON;
use rocket::response::status::NoContent;
use rocket::request::FromParam;
use diesel::result::Error;
use uuid::Uuid;

#[get("/notes", format = "application/json")]
fn notes_get(db: DB) -> Result<JSON<Vec<Note>>, Error> {
    let notes = get_notes(db.conn());
    match notes {
        Ok(notes) => Ok(JSON(notes)),
        Err(err) => Err(err),
    }
}

#[get("/notes/<id>", format = "application/json")]
fn note_get(db: DB, id: &str) -> Result<JSON<Note>, Error> {
    let uuid = Uuid::parse_str(id).unwrap();
    let note = get_note(db.conn(), uuid);
    match note {
        Ok(note) => Ok(JSON(note)),
        Err(err) => Err(err),
    }
}

#[post("/notes", format = "application/json", data = "<note>")]
fn note_create(db: DB, note: NoteData) -> Result<JSON<Note>, Error> {
    let created_note = create_note(db.conn(), note);
    match created_note {
        Ok(note) => Ok(JSON(note)),
        Err(err) => Err(err),
    }
}

#[patch("/notes/<id>", format = "application/json", data = "<note>")]
fn note_edit(db: DB, id: &str, note: NoteData) -> Result<JSON<Note>, Error> {
    let uuid = Uuid::parse_str(id).unwrap();
    let updated_note = update_note(db.conn(), uuid, note);
    match updated_note {
        Ok(note) => Ok(JSON(note)),
        Err(err) => Err(err),
    }
}

#[delete("/notes/<id>")]
fn note_delete(db: DB, id: &str) -> Result<NoContent, Error> {
    let uuid = Uuid::parse_str(id).unwrap();
    match delete_note(db.conn(), uuid) {
        Ok(_) => Ok(NoContent),
        Err(err) => Err(err),
    }
}

fn main() {
    rocket::ignite().mount("/", routes![note_create, notes_get, note_delete, note_edit, note_get]).launch();
}
