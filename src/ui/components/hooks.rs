use std::sync::Arc;

use dioxus::prelude::*;

use crate::{
    core::{
        image::create_image_provider,
        storage::{create_storage, Storage},
    },
    ui::state::AppState,
};

pub fn autofocus_by_id(id: impl Into<String>) {
    let id = id.into();

    use_effect(move || {
        let target_id = id.clone();

        spawn(async move {
            let script = format!(
                r#"
                const element = document.getElementById('{}');
                if (element) {{
                    element.focus();
                }}
                "#,
                target_id
            );

            let _ = document::eval(&script).await;
        });
    });
}

pub fn use_image_provider() {
    let mut app_state: AppState = use_context();
    
    let image_provider = use_resource(move || async move {
        match create_image_provider().await {
            Ok(provider) => Some(provider),
            Err(e) => {
                app_state.notifications.send_error(&format!("Failed to initialize image storage: {}", e));
                None
            }
        }
    });

    use_context_provider(|| image_provider);
}

pub fn use_storage() -> Arc<dyn Storage> {
    let storage = use_hook(|| create_storage().unwrap());
    use_context_provider(|| storage.clone());

    storage
}
