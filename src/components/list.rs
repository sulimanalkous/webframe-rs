use crate::component::Component;
use std::collections::HashMap;

#[derive(Clone)]
pub enum ListType {
    Unordered, // <ul>
    Ordered,   // <ol>
}

pub struct List {
    pub list_type: ListType,
    pub items: Vec<ListItem>,
    pub class: Option<String>,
    pub extra_attrs: HashMap<String, String>,
}
//
// * ListItem
//

pub struct ListItem {
    pub content: String,
    pub class: Option<String>,
}

impl ListItem {
    pub fn new(content: impl Into<String>) -> Self {
        Self {
            content: content.into(),
            class: None,
        }
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }
}

impl List {
    pub fn new(list_type: ListType) -> Self {
        Self {
            list_type,
            items: vec![],
            class: None,
            extra_attrs: HashMap::new(),
        }
    }

    pub fn add(mut self, item: ListItem) -> Self {
        self.items.push(item);
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

impl Component for List {
    fn render(&self) -> String {
        let tag = match self.list_type {
            ListType::Unordered => "ul",
            ListType::Ordered => "ol",
        };

        let mut attrs = String::new();
        for (k, v) in &self.extra_attrs {
            attrs.push_str(&format!(r#" {}="{}""#, k, v));
        }

        let class = self.class.clone().unwrap_or_default();
        let items_html: String = self.items.iter().map(|i| i.render()).collect();

        format!(
            r#"<{tag} class="{class}"{attrs}>{items}</{tag}>"#,
            tag = tag,
            class = class,
            attrs = attrs,
            items = items_html
        )
    }
}

impl Component for ListItem {
    fn render(&self) -> String {
        format!(
            r#"<li class="{}">{}</li>"#,
            self.class.clone().unwrap_or_default(),
            self.content
        )
    }
}
