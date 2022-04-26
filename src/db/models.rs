use serde::Serialize;

use super::schema::metafile;

#[derive(Serialize, Queryable)]
pub struct Metafile {
    pub id: String,
    pub name: String,
    pub path: String,
}

#[derive(Insertable)]
#[table_name = "metafile"]
pub struct NewMetafile<'a> {
    pub id: &'a str,
    pub name: &'a str,
    pub path: &'a str,
}
