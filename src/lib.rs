use openagro_extension::extension::ExtensionMetadata;


pub fn metadata() -> ExtensionMetadata {
    ExtensionMetadata {
        extension_id: "openagro.extension.sales".to_string(),
        name: "Sales".to_string(),
        description: None,
        category: None,
        author: "bluespada".to_string(),
        version: "1.0.0".to_string(),
        depends_on: vec![],
        data: vec![
            include_str!("../assets/some.js").to_string(),
            include_str!("../views/sale.xml").to_string(),
        ],
        application: true,
    }
}