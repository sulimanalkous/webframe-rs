use crate::component::Component;
use std::collections::HashMap;

pub struct OptionItem {
    pub label: String,
    pub value: String,
    pub selected: bool,
}

impl OptionItem {
    pub fn new(label: impl Into<String>, value: impl Into<String>) -> Self {
        Self {
            label: label.into(),
            value: value.into(),
            selected: false,
        }
    }

    pub fn selected(mut self, is_selected: bool) -> Self {
        self.selected = is_selected;
        self
    }
}

pub struct Select {
    pub name: String,
    pub class: Option<String>,
    pub options: Vec<OptionItem>,
    pub extra_attrs: HashMap<String, String>,
}

impl Select {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            class: None,
            options: vec![],
            extra_attrs: HashMap::new(),
        }
    }

    pub fn add(mut self, option: OptionItem) -> Self {
        self.options.push(option);
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

impl Component for Select {
    fn render(&self) -> String {
        let mut attrs = String::new();
        for (k, v) in &self.extra_attrs {
            attrs.push_str(&format!(r#" {}="{}""#, k, v));
        }

        let class = self.class.clone().unwrap_or_default();

        let options_html: String = self.options.iter().map(|opt| {
            let selected = if opt.selected { " selected" } else { "" };
            format!(
                r#"<option value="{}"{}>{}</option>"#,
                opt.value, selected, opt.label
            )
        }).collect();

        format!(
            r#"<select name="{}" class="{}"{}>{}</select>"#,
            self.name,
            class,
            attrs,
            options_html
        )
    }
}

