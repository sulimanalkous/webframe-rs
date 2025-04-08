use crate::component::Component;

pub struct Button {
    pub text: String,
    pub class: Option<String>,
}

impl Button {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            class: None,
        }
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }
}

impl Component for Button {
    fn render(&self) -> String {
        format!(
            r#"<button class="{}">{}</button>"#,
            self.class.clone().unwrap_or_default(),
            self.text
        )
    }
}
