use crate::error::Error;

use scraper::{
    html::Html,
    Selector,
};

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
    document: Html,
    pub modules: Vec<Module>,
    pub structs: Vec<_Struct>,
    pub enums: Vec<_Enum>,
    pub functions: Vec<Function>,
}

impl Documentation {
    pub fn new(document: Html) -> Self {
        Self {
            document,
            modules: vec![],
            structs: vec![],
            enums: vec![],
            functions: vec![],
        }
    }

    pub fn parse_functions(&self)  -> Result<Vec<Function>, Error> {
        Ok(Vec::new())
    }

    pub fn parse_enums(&self)  -> Result<Vec<_Enum>, Error> {
        Ok(Vec::new())
    }

    pub fn parse_modules(&self)  -> Result<Vec<Module>, Error> {
        Ok(Vec::new())
    }

    pub fn parse_structs(&self)  -> Result<Vec<_Struct>, Error> {
        let mut structs = Vec::new();

        let select_structs = Selector::parse(r#"#structs + .item-table .item-name"#)?;

        for _struct in self.document.select(&select_structs) {
            let select_name = Selector::parse(r#"a.struct"#)?;
            let select_deprecated = Selector::parse(r#"span.stab.deprecated"#)?;

            let name = _struct
                .select(&select_name)
                .next()
                .unwrap()
                .inner_html();

            let is_deprecated = _struct
                .select(&select_deprecated)
                .next()
                .is_some();

            let _struct = _Struct {
                name,
                fields: vec![],
                methods: vec![],
                is_deprecated,
            };

            structs.push(_struct);
        }

        Ok(structs)
    }

    pub fn from_raw_html(html: &str) -> Result<Documentation, Error> {
        let document = Html::parse_document(html);
        let mut docs = Documentation::new(document);

        docs.structs    = docs.parse_structs()?;
        docs.modules    = docs.parse_modules()?;
        docs.enums      = docs.parse_enums()?;
        docs.functions  = docs.parse_functions()?;

        Ok(docs)
    }
}
