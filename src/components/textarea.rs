use crate::component::Component;
use std::collections::HashMap;

pub struct Textarea {
    pub name: String,
    pub value: String,
    pub rows: Option<u32>,
    pub placeholder: Option<String>,
    pub class: Option<String>,
    pub extra_attrs: HashMap<String, String>,
}

impl Textarea {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            value: String::new(),
            rows: None,
            placeholder: None,
            class: None,
            extra_attrs: HashMap::new(),
        }
    }

    pub fn value(mut self, val: impl Into<String>) -> Self {
        self.value = val.into();
        self
    }

    pub fn rows(mut self, rows: u32) -> Self {
        self.rows = Some(rows);
        self
    }

    pub fn placeholder(mut self, text: impl Into<String>) -> Self {
        self.placeholder = Some(text.into());
        self
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }

    pub fn with_attr(mut self, key: impl Into<String>, val: impl Into<String>) -> Self {
        self.extra_attrs.insert(key.into(), val.into());
        self
    }
}

impl Component for Textarea {
    fn render(&self) -> String {
        let mut attrs = String::new();
        for (k, v) in &self.extra_attrs {
            attrs.push_str(&format!(r#" {}="{}""#, k, v));
        }

        let class = self.class.clone().unwrap_or_default();
        let placeholder = self.placeholder.clone().unwrap_or_default();
        let rows = self
            .rows
            .map_or(String::new(), |r| format!(r#" rows="{}""#, r));

        format!(
            r#"<textarea name="{}" class="{}" placeholder="{}"{}{}>{}</textarea>"#,
            self.name, class, placeholder, rows, attrs, self.value
        )
    }
}
