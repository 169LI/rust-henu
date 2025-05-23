use utils::{id_gen, date};

#[test]
fn test_generate_id() {
    let id = id_gen::generate_id();
    assert_eq!(id.len(), 4);
    assert!(id.chars().all(|c| c.is_ascii_alphanumeric()));
}

#[test]
fn test_format_date() {
    let date = date::format_date();
    assert_eq!(date.len(), 10);
    assert!(date.contains('-'));
} 