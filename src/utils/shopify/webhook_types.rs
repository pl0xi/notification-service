use serde::{Deserialize, Serialize};
use std::fmt::Debug;

#[derive(Deserialize, Serialize, Debug)]
pub struct Customer {
    pub email: String,
    pub first_name: String,
    pub last_name: String,
}
