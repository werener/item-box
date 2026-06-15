use fully_pub::fully_pub;
use serde::{Deserialize, Serialize};
use std::fmt;
use uuid::Uuid;

/// Tool represents the tool with all its information attached.
#[derive(Default, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[fully_pub]
struct Tool {
    id: Uuid,
    name: String,
    location: String,
    price: f64,
    folder: Option<Uuid>,
    tags: Vec<Uuid>,
}
impl Tool {
    pub fn new(
        name: String,
        location: String,
        price: f64,
        folder: Option<Uuid>,
        tags: Vec<Uuid>,
    ) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            location,
            price,
            folder,
            tags,
        }
    }
}

/// Folder represents a collection of tools, organized into one group by an arbitrary feature.
///
/// Multiple tools can have the same folder.
/// One tool can have only one folder.
#[derive(Default, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[fully_pub]
struct Folder {
    id: Uuid,
    name: String,
}
impl Folder {
    pub fn new(name: String) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
        }
    }
}

/// Tag represents an affiliation of a tool to a particular group.
///
/// Multiple tools can have the same tag.
/// One tool can have multiple tags.
#[derive(Default, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[fully_pub]
struct Tag {
    id: Uuid,
    name: String,
    color: Color,
}
impl Tag {
    pub fn new(name: String, color: Color) -> Self {
        Self {
            id: Uuid::new_v4(),
            name,
            color,
        }
    }
}

/// Represents a color in RGB.
#[derive(Default, Clone, PartialEq, Debug, Deserialize, Serialize)]
#[fully_pub]
struct Color {
    r: u8,
    g: u8,
    b: u8,
}
impl Color {
    /// Returns hex representation of the color (without leading #)
    pub fn to_hex(&self) -> String {
        format!("{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02X}{:02X}{:02X}", self.r, self.g, self.b)
    }
}
