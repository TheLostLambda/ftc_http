use serde::Deserialize;

#[derive(Deserialize)]
pub struct Files {
    pub src: Vec<String>,
}
