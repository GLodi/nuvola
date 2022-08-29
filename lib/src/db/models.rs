#[derive(Eq, PartialEq, Debug, Serialize, Deserialize, Clone)]
pub struct Metafile {
    pub id: String,
    pub name: String,
    pub path: String,
}
