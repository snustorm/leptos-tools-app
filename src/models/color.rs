use serde::{Deserialize, Serialize};

#[cfg(feature = "ssr")]
use sqlx::FromRow;

#[cfg_attr(feature = "ssr", derive(FromRow))]
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct Color {
    pub id: i32,
    pub name: String,
    pub hex_xcode: String,
}