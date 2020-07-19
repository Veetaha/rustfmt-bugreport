
#[path = ""]
mod foo {

    #[path = "foo.bar.rs"]
    pub(crate) mod bar;
}

pub use foo::bar::foo_bar;
