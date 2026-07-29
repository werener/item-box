// src/core/image.rs
use crate::error::StorageError;
use async_trait::async_trait;
use std::sync::Arc;
use uuid::Uuid;

const PROJECT_NAME: &str =
    option_env!("PROJECT_NAME").expect("env variable 'PROJECT_NAME' not found (image)");

#[async_trait(?Send)]
pub trait ImageProvider {
    /// Saves raw image bytes and returns a unique ID for it.
    async fn save_image(&self, data: &[u8]) -> Result<Uuid, StorageError>;

    /// Retrieves a renderable URI for the UI.
    /// - Desktop: Returns a local file path (file://...)
    /// - Web: Fetches from IndexedDB and returns a temporary blob: URL
    async fn get_image_uri(&self, id: Uuid) -> Result<Option<String>, StorageError>;

    /// Deletes the image data.
    async fn delete_image(&self, id: Uuid) -> Result<(), StorageError>;
}

/// Desktop implementation of ImageProvider trait.
///
/// Works with image data, stored locally.
#[cfg(not(target_arch = "wasm32"))]
pub struct DesktopImageProvider {
    image_dir: std::path::PathBuf,
}

#[cfg(not(target_arch = "wasm32"))]
impl DesktopImageProvider {
    pub async fn new() -> Result<Self, StorageError> {
        let image_dir = directories::ProjectDirs::from("com", PROJECT_NAME, PROJECT_NAME)
            .map(|dirs| dirs.data_dir().join("images"))
            .unwrap_or_else(|| std::path::PathBuf::from("./images"));

        tokio::fs::create_dir_all(&image_dir).await?;

        Ok(Self { image_dir })
    }
}

#[cfg(not(target_arch = "wasm32"))]
#[async_trait(?Send)]
impl ImageProvider for DesktopImageProvider {
    async fn save_image(&self, data: &[u8]) -> Result<Uuid, StorageError> {
        let id = Uuid::new_v4();
        let file_path = self.image_dir.join(id.to_string());

        // Updated: Non-blocking async write
        tokio::fs::write(&file_path, data).await?;

        Ok(id)
    }

    async fn get_image_uri(&self, id: Uuid) -> Result<Option<String>, StorageError> {
        let file_path = self.image_dir.join(id.to_string());

        if tokio::fs::try_exists(&file_path).await? {
            let canonical_path = tokio::fs::canonicalize(&file_path).await?;
            let path_str = canonical_path.to_string_lossy().replace('\\', "/");
            Ok(Some(format!("file://{}", path_str)))
        } else {
            Ok(None)
        }
    }

    async fn delete_image(&self, id: Uuid) -> Result<(), StorageError> {
        let file_path = self.image_dir.join(id.to_string());

        if tokio::fs::try_exists(&file_path).await? {
            tokio::fs::remove_file(file_path).await?;
        }
        Ok(())
    }
}

/// Web implementation of ImageProvider trait.
///
/// Works with image data, stored in an IndexedDB.
#[cfg(target_arch = "wasm32")]
pub struct WebImageProvider {
    db_name: String,
    store_name: String,
}

#[cfg(target_arch = "wasm32")]
impl WebImageProvider {
    pub fn new() -> Self {
        Self {
            db_name: PROJECT_NAME.to_string(),
            store_name: "images".to_string(),
        }
    }

    async fn get_db(&self) -> Result<idb::Database, StorageError> {
        // Required in idb 0.6 to access the .database() method on VersionChangeEvent
        use idb::DatabaseEvent;

        let factory =
            idb::Factory::new().map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;
        let mut req = factory
            .open(&self.db_name, Some(1))
            .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;

        let store_name = self.store_name.clone();
        req.on_upgrade_needed(move |evt| {
            let db = evt
                .database()
                .expect("Failed to get database during upgrade");

            // Use .contains() instead of .any() for cleaner Vec<String> checking
            if !db.store_names().contains(&store_name) {
                let mut params = idb::ObjectStoreParams::new();
                params.key_path(Some(idb::KeyPath::new_single("id")));
                db.create_object_store(&store_name, params)
                    .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))
                    .ok();
            }
        });

        let db = req
            .await
            .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;
        Ok(db)
    }
}

#[cfg(target_arch = "wasm32")]
#[async_trait(?Send)]
impl ImageProvider for WebImageProvider {
    async fn save_image(&self, data: &[u8]) -> Result<Uuid, StorageError> {
        let id = Uuid::new_v4();
        let db = self.get_db().await?;
        let tx = db
            .transaction(&[&self.store_name], idb::TransactionMode::ReadWrite)
            .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;
        let store = tx
            .object_store(&self.store_name)
            .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;

        let js_array = js_sys::Uint8Array::from(data);
        let js_value: wasm_bindgen::JsValue = js_array.into();
        let record = js_sys::Object::new();
        js_sys::Reflect::set(&record, &"id".into(), &id.to_string().into()).ok();
        js_sys::Reflect::set(&record, &"data".into(), &js_value).ok();

        // put() requires the value and an optional key (None since we use keyPath)
        store
            .put(&record, None)
            .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;

        // In idb 0.6, tx.await directly returns Result<TransactionResult, Error>
        tx.await
            .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;
        Ok(id)
    }

    async fn get_image_uri(&self, id: Uuid) -> Result<Option<String>, StorageError> {
        let db = self.get_db().await?;
        let tx = db
            .transaction(&[&self.store_name], idb::TransactionMode::ReadOnly)
            .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;
        let store = tx
            .object_store(&self.store_name)
            .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;

        // Query in idb 0.6 requires a JsValue, which implements Into<Query>
        let query: wasm_bindgen::JsValue = id.to_string().into();
        let result = store
            .get(query)
            .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;

        // store.get().await returns Result<Option<JsValue>, Error>
        let result = result
            .await
            .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;

        // Properly handle the Option<JsValue>
        let result = match result {
            Some(val) => val,
            None => return Ok(None),
        };

        let data_val = js_sys::Reflect::get(&result, &"data".into())
            .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;

        let uint8_array = js_sys::Uint8Array::new(&data_val);
        let blob = web_sys::Blob::new_with_u8_array_sequence(&js_sys::Array::of1(&uint8_array))
            .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;

        let uri = web_sys::Url::create_object_url_with_blob(&blob)
            .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;

        Ok(Some(uri))
    }

    async fn delete_image(&self, id: Uuid) -> Result<(), StorageError> {
        let db = self.get_db().await?;
        let tx = db
            .transaction(&[&self.store_name], idb::TransactionMode::ReadWrite)
            .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;
        let store = tx
            .object_store(&self.store_name)
            .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;

        let query: wasm_bindgen::JsValue = id.to_string().into();
        store
            .delete(query)
            .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;

        tx.await
            .map_err(|e| StorageError::WebStorage(format!("{:?}", e)))?;
        Ok(())
    }
}

// Factory Function
pub async fn create_image_provider() -> Result<Arc<dyn ImageProvider>, StorageError> {
    #[cfg(target_arch = "wasm32")]
    {
        Ok(Arc::new(WebImageProvider::new()))
    }

    #[cfg(not(target_arch = "wasm32"))]
    {
        Ok(Arc::new(DesktopImageProvider::new().await?))
    }
}
