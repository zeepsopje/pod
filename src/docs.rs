use crate::error::Error;

use scraper::{html::Html, Selector};

#[derive(Debug)]
pub struct MetaData {
    pub crate_name: String,
    pub versions: Vec<String>,
    pub dependencies: Vec<String>,
}

#[derive(Debug)]
pub struct Field {
    key: String,
    value: String,
}

#[derive(Debug)]
pub struct Method {
}

#[derive(Debug)]
pub struct Module {
    pub modules: Vec<Box<Module>>
}

#[derive(Debug)]
pub struct _Struct {
    pub name: String,
    pub fields: Vec<Field>,
    pub methods: Vec<Method>,
    pub is_deprecated: bool,
}

#[derive(Debug)]
pub struct _Enum {}
#[derive(Debug)]
pub struct Function {}

#[derive(Debug)]
pub struct Documentation {
    pub meta_data: MetaData,
    pub modules: Vec<Module>,
    pub structs: Vec<_Struct>,
    pub enums: Vec<_Enum>,
    pub functions: Vec<Function>,
}

impl Documentation {
    pub fn new(meta_data: MetaData) -> Self {
        Self {
            meta_data,
            modules: Vec::new(),
            structs: Vec::new(),
            enums: Vec::new(),
            functions: Vec::new(),
        }
    }

    pub fn from_raw_html(html: &str) -> Result<Documentation, Error> {
        let document = Html::parse_document(html);

        // Metadata
        let select_meta_data = Selector::parse(r#".package-details-menu"#)?;
        let select_crate_name = Selector::parse(r#"#crate-title"#)?;
        let select_dependencies = Selector::parse(r#".pure-g.menu-item-divided > :first-child .pure-menu-list .pure-menu-list li a"#)?;
        let select_versions = Selector::parse(r#".pure-g.menu-item-divided > :last-child .pure-menu-list .pure-menu-list li a"#)?;

        let mut crate_name = String::new();
        let mut versions = Vec::new();
        let mut dependencies = Vec::new();

        for node in document.select(&select_versions) {
            println!("hello there");
            versions.push(node.inner_html());
        }

        for dependency in document.select(&select_dependencies) {
            let dependency = dependency
                .attr("href")
                .unwrap()
                .replace("/", "")
                .trim()
                .to_owned();
            dependencies.push(dependency);
        }

        let meta_data = MetaData {
            crate_name,
            versions,
            dependencies,
        };

        let mut docs = Documentation::new(meta_data);

        // Structs
        let select_structs = Selector::parse(r#"#structs + .item-table .item-name > a.struct"#)?;
        for _struct in document.select(&select_structs) {
            let _struct = _Struct {
                name: _struct.inner_html(),
                fields: vec![],
                methods: vec![],
                is_deprecated: false,
            };

            docs.structs.push(_struct);
        }

        Ok(docs)
    }
}
