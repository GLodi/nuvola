pub mod models;
pub mod schema;

use diesel::prelude::*;
use dotenv::dotenv;
use std::env;
use uuid::Uuid;

use models::*;
use schema::metafile;
use schema::metafile::dsl::*;

fn establish_connection() -> SqliteConnection {
    dotenv().ok();

    let database_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    SqliteConnection::establish(&database_url)
        .unwrap_or_else(|_| panic!("Error connecting to {}", database_url))
}

pub fn get_metafiles() -> Vec<Metafile> {
    let connection = establish_connection();

    metafile
        .limit(5)
        .load::<Metafile>(&connection)
        .expect("Error loading metafiles")
}

pub fn create_metafile(t: &str, b: &str) -> String {
    let connection = establish_connection();

    let uuid = Uuid::new_v4().as_hyphenated().to_string();

    let new_metafile = NewMetafile {
        id: &uuid,
        name: t,
        path: b,
    };

    diesel::insert_into(metafile::table)
        .values(&new_metafile)
        .execute(&connection)
        .expect("Error saving new metafile");

    uuid
}
