use lazy_static::lazy_static;

lazy_static! {
    pub(crate) static ref COMPILED_TEMPLATES: tera::Tera = {
        tera::Tera::new(concat!(env!("CARGO_MANIFEST_DIR"), "/pages/**/*")).unwrap()
    };
}