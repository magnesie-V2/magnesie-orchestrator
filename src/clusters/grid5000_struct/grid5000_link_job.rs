extern crate serde;
use serde::{Deserialize};

#[derive(Deserialize, Debug)]
/// Representation of a Link
pub struct LinkJob {
    pub rel : String,
    pub href : String,
    pub r#type : String,
}