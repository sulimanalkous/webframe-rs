// src/components/form.rs
use crate::component::Component;

pub struct Form {
    children: Vec<Box<dyn Component>>,
    method: String,
    action: String,
}

impl Form {
    pub fn new() -> Self {
        Self {
            children: Vec::new(),
            method: "POST".into(),
            action: "".into(),
        }
    }

    pub fn method(mut self, method: impl Into<String>) -> Self {
        self.method = method.into();
        self
    }

    pub fn action(mut self, action: impl Into<String>) -> Self {
        self.action = action.into();
        self
    }

    pub fn add(mut self, child: impl Component + 'static) -> Self {
        self.children.push(Box::new(child));
        self
    }
}

impl Component for Form {
    fn render(&self) -> String {
        let inner_html: String = self
            .children
            .iter()
            .map(|c| c.render())
            .collect::<Vec<_>>()
            .join("\n");
        format!(
            r#"<form method="{}" action="{}">{}</form>"#,
            self.method, self.action, inner_html
        )
    }
}
