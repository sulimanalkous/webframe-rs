use crate::component::Component;

pub struct Raw(pub String);

impl Component for Raw {
    fn render(&self) -> String {
        self.0.clone()
    }
}
