// src/core/builder.rs
use super::models::Item;
use uuid::Uuid;

/// Builder pattern for constructing Tool.
#[derive(Default, Debug)]
pub struct ItemBuilder {
    name: String,
    location: Option<String>,
    description: Option<String>,
    price: f64,
    folder: Option<Uuid>,
    tags: Vec<Uuid>,
}

#[fully_pub::fully_pub]
impl ItemBuilder {
    fn new<S: Into<String>>(name: S) -> Self {
        Self {
            name: name.into(),
            ..Default::default()
        }
    }

    fn location<S: Into<String>>(mut self, location: S) -> Self {
        let loc = location.into();
        self.location = if loc.trim().is_empty() {
            None
        } else {
            Some(loc)
        };
        self
    }

    fn description<S: Into<String>>(mut self, description: S) -> Self {
        let desc = description.into();
        self.description = if desc.trim().is_empty() {
            None
        } else {
            Some(desc)
        };
        self
    }

    fn price(mut self, price: f64) -> Self {
        self.price = price;
        self
    }

    fn folder(mut self, folder: Option<Uuid>) -> Self {
        self.folder = folder;
        self
    }

    fn tags(mut self, tags: Vec<Uuid>) -> Self {
        self.tags = tags;
        self
    }

    /// Builds the resulting Item struct.
    fn build(self) -> Item {
        Item {
            id: Uuid::new_v4(),
            name: self.name,
            location: self.location,
            description: self.description,
            price: self.price,
            folder: self.folder,
            tags: self.tags,
        }
    }
}
