use crate::component::Component;
use std::collections::HashMap;

#[derive(Clone)]
pub enum FieldType {
    Text,
    Email,
    Password,
    Number,
    Search,
    Tel,
    Url,
    Hidden,
    Date,
    Time,
    DatetimeLocal,
    File,
    Checkbox,
    Radio,
}

impl FieldType {
    pub fn as_str(&self) -> &'static str {
        match self {
            FieldType::Text => "text",
            FieldType::Email => "email",
            FieldType::Password => "password",
            FieldType::Number => "number",
            FieldType::Search => "search",
            FieldType::Tel => "tel",
            FieldType::Url => "url",
            FieldType::Hidden => "hidden",
            FieldType::Date => "date",
            FieldType::Time => "time",
            FieldType::DatetimeLocal => "datetime-local",
            FieldType::File => "file",
            FieldType::Checkbox => "checkbox",
            FieldType::Radio => "radio",
        }
    }
}

pub struct Input {
    pub name: String,
    pub input_type: FieldType,
    pub value: Option<String>,
    pub placeholder: Option<String>,
    pub class: Option<String>,
    pub extra_attrs: HashMap<String, String>,
}

impl Input {
    pub fn new(name: impl Into<String>) -> Self {
        Self {
            name: name.into(),
            input_type: FieldType::Text,
            value: None,
            placeholder: None,
            class: None,
            extra_attrs: HashMap::new(),
        }
    }

    pub fn input_type(mut self, input_type: FieldType) -> Self {
        self.input_type = input_type;
        self
    }
    pub fn value(mut self, val: impl Into<String>) -> Self {
        self.value = Some(val.into());
        self
    }
    pub fn placeholder(mut self, ph: impl Into<String>) -> Self {
        self.placeholder = Some(ph.into());
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

impl Component for Input {
    fn render(&self) -> String {
        let mut attrs = String::new();
        for (k, v) in &self.extra_attrs {
            attrs.push_str(&format!(r#" {}="{}""#, k, v));
        }
        format!(
            r#"<input type="{}" name="{}" value="{}" placeholder="{}" class="{}"{} />"#,
            self.name,
            self.input_type.as_str(),
            self.value.clone().unwrap_or_default(),
            self.placeholder.clone().unwrap_or_default(),
            self.class.clone().unwrap_or_default(),
            attrs
        )
    }
}
