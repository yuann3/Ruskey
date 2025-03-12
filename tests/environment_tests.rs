use ruskey::environment::Environment;
use ruskey::object::Integer;

#[test]
fn test_environment() {
    let mut env = Environment::new();

    // Create a value to store
    let val = Box::new(Integer::new(5));

    // Store the value with the name "x"
    env.set("x".to_string(), val);

    // Retrieve the value and verify it's correct
    let retrieved = env.get(&"x".to_string());
    assert!(retrieved.is_some());

    let int_obj = retrieved.unwrap();
    let integer = int_obj.as_any().downcast_ref::<Integer>().unwrap();
    assert_eq!(integer.value, 5);
}
