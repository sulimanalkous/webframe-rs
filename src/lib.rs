pub mod component;
pub mod components {
    pub mod button;
    pub mod form;
    pub mod input;
    pub mod list;
    pub mod raw;
    pub mod select;
    pub mod table;
    pub mod textarea;
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}
