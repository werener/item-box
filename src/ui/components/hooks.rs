use std::sync::Arc;

use dioxus::prelude::*;

use crate::{
    core::{
        image::{create_image_provider, ImageProvider},
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
    let image_provider: Signal<Option<Arc<dyn ImageProvider>>> = use_signal(|| None);
    let app_state: AppState = use_context();

    use_context_provider(|| image_provider);

    use_effect(move || {
        let mut provider_signal = image_provider;
        let mut app_state = app_state;

        spawn(async move {
            if let Ok(provider) = create_image_provider().await {
                provider_signal.set(Some(provider));
            } else {
                app_state
                    .notifications
                    .send_error("Cannot create a folder for storing images.");
            }
        });
    });
}

pub fn use_storage() -> Arc<dyn Storage> {
    let storage = use_hook(|| create_storage().unwrap());
    use_context_provider(|| storage.clone());

    storage
}
