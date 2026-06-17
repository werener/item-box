use super::inventory::Inventory;

pub trait Storage {
    fn save(&self, inventory: &Inventory) -> Result<(), Box<dyn std::error::Error>>;
    fn load(&self) -> Result<Inventory, Box<dyn std::error::Error>>;
}