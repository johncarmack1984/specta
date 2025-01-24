use std::collections::HashMap;

use specta::Type;
use specta_typescript::Typescript;

#[derive(Type)]
pub struct TypeOne {
    pub field1: String,
    pub field2: i32,

    // Overriding the field type doesn't effect serde so your JSON and types may not match but if you know what your doing this is useful
    #[specta(type = String)]
    pub override_type: i32,
}

#[derive(Type)]
pub struct GenericType<A> {
    pub my_field: String,
    pub generic: A,
}

#[derive(Type, Hash)]
pub enum MyEnum {
    A,
    B,
    C,
}

#[derive(Type)]
pub struct Something {
    a: HashMap<MyEnum, i32>,
}

fn main() {
    let ts_str = specta_typescript::export::<TypeOne>(&Typescript::default()).unwrap();
    println!("{ts_str}");
    assert_eq!(
        ts_str,
        "export type TypeOne = { field1: string; field2: number; override_type: string }"
            .to_string()
    );

    let ts_str = specta_typescript::export::<GenericType<()>>(&Typescript::default()).unwrap();
    println!("{ts_str}");
    assert_eq!(
        ts_str,
        "export type GenericType<A> = { my_field: string; generic: A }".to_string()
    );

    let ts_str = specta_typescript::export::<MyEnum>(&Typescript::default()).unwrap();
    println!("{ts_str}");
    assert_eq!(
        ts_str,
        r#"export type MyEnum = "A" | "B" | "C""#.to_string()
    );

    let ts_str = specta_typescript::export::<Something>(&Typescript::default()).unwrap();
    println!("{ts_str}");
    assert_eq!(
        ts_str,
        r#"export type Something = { a: Partial<{ [key in MyEnum]: number }>"#.to_string()
    );
}
