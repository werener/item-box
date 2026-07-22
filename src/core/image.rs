use std::sync::Arc;

// src/core/image.rs
use crate::error::StorageError;
use async_trait::async_trait;
use uuid::Uuid;

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
        let image_dir = directories::ProjectDirs::from("com", "vtoolbox", "vtoolbox")
            .map(|dirs| dirs.data_dir().join("images"))
            .unwrap_or_else(|| std::path::PathBuf::from("./images"));

        // Updated: Non-blocking directory creation
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
        
        // Updated: Non-blocking existence check and canonicalization
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
        
        // Updated: Non-blocking existence check and removal
        if tokio::fs::try_exists(&file_path).await? {
            tokio::fs::remove_file(file_path).await?;
        }
        Ok(())
    }
}

/// Web implementation of ImageProvider trait.
///
/// Works with image data, stored stored in an indexed DB.
#[cfg(target_arch = "wasm32")]
pub struct WebImageProvider {
    db_name: String,
    store_name: String,
}

#[cfg(target_arch = "wasm32")]
impl WebImageProvider {
    pub fn new() -> Self {
        Self {
            db_name: "vtoolbox_images".to_string(),
            store_name: "images".to_string(),
        }
    }

    async fn get_db(&self) -> Result<idb::Database, StorageError> {
        let factory = idb::Factory::new().map_err(|e| StorageError::WebStorage(e.to_string()))?;

        let db = factory
            .open(&self.db_name, Some(1), |evt| {
                let db = evt.database();
                if !db.object_store_names().any(|name| name == self.store_name) {
                    let mut params = idb::ObjectStoreParams::new();
                    params.key_path(Some(&idb::KeyPath::from("id")));
                    db.create_object_store(&self.store_name, params)
                        .map_err(|e| StorageError::WebStorage(e.to_string()))
                        .ok();
                }
                Ok(())
            })
            .await
            .map_err(|e| StorageError::WebStorage(e.to_string()))?;

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
            .map_err(|e| StorageError::WebStorage(e.to_string()))?;

        let store = tx
            .object_store(&self.store_name)
            .map_err(|e| StorageError::WebStorage(e.to_string()))?;

        // Convert Rust slice to Js Uint8Array for IndexedDB
        let js_array = js_sys::Uint8Array::from(data);
        let js_value: wasm_bindgen::JsValue = js_array.into();

        let mut record = js_sys::Object::new();
        js_sys::Reflect::set(&record, &"id".into(), &id.to_string().into()).ok();
        js_sys::Reflect::set(&record, &"data".into(), &js_value).ok();

        store
            .put(&record)
            .map_err(|e| StorageError::WebStorage(e.to_string()))?;

        tx.await
            .into_result()
            .map_err(|e| StorageError::WebStorage(e.to_string()))?;

        Ok(id)
    }

    async fn get_image_uri(&self, id: Uuid) -> Result<Option<String>, StorageError> {
        let db = self.get_db().await?;
        let tx = db
            .transaction(&[&self.store_name], idb::TransactionMode::ReadOnly)
            .map_err(|e| StorageError::WebStorage(e.to_string()))?;

        let store = tx
            .object_store(&self.store_name)
            .map_err(|e| StorageError::WebStorage(e.to_string()))?;

        let result = store
            .get(&id.to_string().into())
            .map_err(|e| StorageError::WebStorage(e.to_string()))?;

        let result = result
            .await
            .into_result()
            .map_err(|e| StorageError::WebStorage(e.to_string()))?;

        if result.is_null() || result.is_undefined() {
            return Ok(None);
        }

        // Extract the Uint8Array from the stored object
        let data_val = js_sys::Reflect::get(&result, &"data".into())
            .map_err(|e| StorageError::WebStorage(e.to_string()))?;

        let uint8_array = js_sys::Uint8Array::from(&data_val);
        let blob = web_sys::Blob::new_with_u8_array_sequence(&js_sys::Array::of1(&uint8_array))
            .map_err(|e| StorageError::WebStorage(e.to_string()))?;

        let uri = web_sys::Url::create_object_url_with_blob(&blob)
            .map_err(|e| StorageError::WebStorage(e.to_string()))?;

        Ok(Some(uri))
    }

    async fn delete_image(&self, id: Uuid) -> Result<(), StorageError> {
        let db = self.get_db().await?;
        let tx = db
            .transaction(&[&self.store_name], idb::TransactionMode::ReadWrite)
            .map_err(|e| StorageError::WebStorage(e.to_string()))?;

        let store = tx
            .object_store(&self.store_name)
            .map_err(|e| StorageError::WebStorage(e.to_string()))?;

        store
            .delete(&id.to_string().into())
            .map_err(|e| StorageError::WebStorage(e.to_string()))?;

        tx.await
            .into_result()
            .map_err(|e| StorageError::WebStorage(e.to_string()))?;

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