use std::fmt;

// src/core/models.rs
use crate::error::ItemError::{self, InvalidName, InvalidPrice};
use fully_pub::fully_pub;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Item represents any item with all its information attached.
#[derive(Clone, PartialEq, Debug, Deserialize, Serialize)]
#[fully_pub]
struct Item {
    id: Uuid,
    name: String,
    location: Option<String>,
    description: Option<String>,
    price: f64,
    folder: Option<Uuid>,
    tags: Vec<Uuid>,
    images: Vec<Uuid>,
}

impl Default for Item {
    fn default() -> Self {
        Self {
            id: Uuid::new_v4(),
            location: Option::default(),
            description: Option::default(),
            name: String::default(),
            price: f64::default(),
            folder: Option::default(),
            tags: Vec::default(),
            images: Vec::default()
        }
    }
}

#[fully_pub]
impl Item {
    fn is_valid(&self) -> Result<(), Vec<ItemError>> {
        let mut errors: Vec<ItemError> = Vec::new();

        if self.name.trim().is_empty() {
            errors.push(InvalidName);
        }
        if self.price < 0.0 {
            errors.push(InvalidPrice);
        }

        if errors.is_empty() {
            Ok(())
        } else {
            Err(errors)
        }
    }

    fn name(&self) -> String {
        self.name.clone()
    }

    fn location(&self) -> String {
        if let Some(location) = self.location.clone() {
            return location;
        } else {
            return "".to_string();
        }
    }
    fn description(&self) -> String {
        if let Some(description) = self.description.clone() {
            return description;
        } else {
            return "".to_string();
        }
    }
}

/// Folder represents a collection of items, organized into one group by an arbitrary feature.
///
/// Multiple items can have the same folder.
/// One item can have only one folder.
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
    pub fn name(&self) -> String {
        self.name.clone()
    }
}

/// Tag represents an affiliation of a item to a particular group.
///
/// Multiple items can have the same tag.
/// One item can have multiple tags.
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
        format!("{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
    
    /// Parses a hex string into a Color
    pub fn from_hex(hex: &str) -> Result<Self, &'static str> {
        let hex = hex.strip_prefix('#').unwrap_or(hex);
        if hex.len() != 6 {
            return Err("Invalid hex length: expected 6 characters");
        }

        let r = u8::from_str_radix(&hex[0..2], 16).map_err(|_| "Invalid red component")?;
        let g = u8::from_str_radix(&hex[2..4], 16).map_err(|_| "Invalid green component")?;
        let b = u8::from_str_radix(&hex[4..6], 16).map_err(|_| "Invalid blue component")?;

        Ok(Self { r, g, b })
    }
}
impl fmt::Display for Color {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{:02x}{:02x}{:02x}", self.r, self.g, self.b)
    }
}
