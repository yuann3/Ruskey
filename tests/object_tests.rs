use ruskey::object::{Boolean, Integer, Null, Object, ObjectType};

#[test]
fn test_object_types() {
    // Test Integer
    let integer = Integer::new(5);
    assert_eq!(integer.type_(), ObjectType::Integer);
    assert_eq!(integer.inspect(), "5");

    // Test Boolean
    let boolean = Boolean::new(true);
    assert_eq!(boolean.type_(), ObjectType::Boolean);
    assert_eq!(boolean.inspect(), "true");

    // Test Null
    let null = Null::new();
    assert_eq!(null.type_(), ObjectType::Null);
    assert_eq!(null.inspect(), "null");
}
