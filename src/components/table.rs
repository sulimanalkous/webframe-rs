use crate::component::Component;
use std::collections::HashMap;

pub struct Table {
    pub headers: Vec<String>,
    pub rows: Vec<TableRow>,
    pub class: Option<String>,
    pub extra_attrs: HashMap<String, String>,
}

impl Table {
    pub fn new() -> Self {
        Self {
            headers: vec![],
            rows: vec![],
            class: None,
            extra_attrs: HashMap::new(),
        }
    }

    pub fn headers(mut self, headers: Vec<&str>) -> Self {
        self.headers = headers.into_iter().map(String::from).collect();
        self
    }

    pub fn add_row(mut self, row: TableRow) -> Self {
        self.rows.push(row);
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

impl Component for Table {
    fn render(&self) -> String {
        let class = self.class.clone().unwrap_or_default();

        let mut attrs = String::new();
        for (k, v) in &self.extra_attrs {
            attrs.push_str(&format!(r#" {}="{}""#, k, v));
        }

        let thead = if !self.headers.is_empty() {
            let headers_html = self
                .headers
                .iter()
                .map(|h| format!(r#"<th>{}</th>"#, h))
                .collect::<String>();
            format!(r#"<thead><tr>{}</tr></thead>"#, headers_html)
        } else {
            String::new()
        };

        let rows_html = self
            .rows
            .iter()
            .map(|r| r.render())
            .collect::<Vec<_>>()
            .join("");

        format!(
            r#"<table class="{}"{}>{}{}</table>"#,
            class, attrs, thead, rows_html
        )
    }
}

//
// 🔹 TableRow
//
pub struct TableRow {
    pub cells: Vec<TableCell>,
    pub class: Option<String>,
}

impl TableRow {
    pub fn new() -> Self {
        Self {
            cells: vec![],
            class: None,
        }
    }

    pub fn add_cell(mut self, cell: TableCell) -> Self {
        self.cells.push(cell);
        self
    }

    pub fn class(mut self, class: impl Into<String>) -> Self {
        self.class = Some(class.into());
        self
    }
}

impl Component for TableRow {
    fn render(&self) -> String {
        let class = self.class.clone().unwrap_or_default();
        let cells_html = self
            .cells
            .iter()
            .map(|c| c.render())
            .collect::<Vec<_>>()
            .join("");
        format!(r#"<tr class="{}">{}</tr>"#, class, cells_html)
    }
}

//
// 🔹 TableCell
//
pub struct TableCell {
    pub content: String,
    pub class: Option<String>,
}

impl TableCell {
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

impl Component for TableCell {
    fn render(&self) -> String {
        let class = self.class.clone().unwrap_or_default();
        format!(r#"<td class="{}">{}</td>"#, class, self.content)
    }
}
