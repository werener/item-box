use dioxus::prelude::*;

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
