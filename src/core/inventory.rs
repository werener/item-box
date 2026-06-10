use super::models::*;
use fully_pub::fully_pub;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

/// Inventory represents a collection of all availiable models
#[derive(Clone, Debug, Default, Serialize, Deserialize)]
#[fully_pub]
struct Inventory {
    tools: Vec<Tool>,
    folders: Vec<Folder>,
    tags: Vec<Tag>,
}
