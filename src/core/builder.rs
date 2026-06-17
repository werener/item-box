use super::models::Tool;
use uuid::Uuid;

/// Builder pattern for constructing Tool.
#[derive(Default, Debug)]
pub struct ToolBuilder {
    name: String,
    location: Option<String>,
    description: Option<String>,
    price: f64,
    folder: Option<Uuid>,
    tags: Vec<Uuid>,
}

impl ToolBuilder {
    pub fn new(name: String) -> Self {
        Self {
            name,
            ..Default::default()
        }
    }

    pub fn location(mut self, location: String) -> Self {
        self.location = if location.trim().is_empty() {
            None
        } else {
            Some(location)
        };
        self
    }

    pub fn description(mut self, description: String) -> Self {
        self.description = if description.trim().is_empty() {
            None
        } else {
            Some(description)
        };
        self
    }

    pub fn price(mut self, price: f64) -> Self {
        if price < 0.0 {
            return self;
        }
        self.price = price;
        self
    }

    pub fn folder(mut self, folder: Option<Uuid>) -> Self {
        self.folder = folder;
        self
    }

    pub fn tags(mut self, tags: Vec<Uuid>) -> Self {
        self.tags = tags;
        self
    }

    /// Builds the resulting Tool struct.
    pub fn build(self) -> Tool {
        Tool {
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
