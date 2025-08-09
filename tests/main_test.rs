use prop_reader::PropReader;

const VALID_FILENAME: &str = "./tests/valid.properties";
const INVALID_FILENAME: &str = "./tests/invalid.properties";
const SHORT_FILENAME: &str = "./tests/small_valid.properties";
const LARGE_FILENAME: &str = "./tests/large.properties";

#[test]
fn new_test() {
    let properties: PropReader = PropReader::new(VALID_FILENAME);
    let test1 = properties.exists("equals.no.quotes");
    let test2 = properties.exists("fake.property");

    assert_eq!(test1, true);
    assert_eq!(test2, false);
}

#[test]
fn prop_with_equals() {
    let properties: PropReader = PropReader::new(VALID_FILENAME);
    let test1 = properties.get("equals.no.quotes");
    let test2 = properties.get("equals.with.quotes");
    let test3 = properties.get("equals.with.spacing.no.quotes");
    let test4 = properties.get("equals.with.spacing.and.quotes");

    assert_eq!(test1, "pass");
    assert_eq!(test2, "pass");
    assert_eq!(test3, "pass");
    assert_eq!(test4, "pass");
}

#[test]
fn prop_with_semicolon() {
    let properties: PropReader = PropReader::new(VALID_FILENAME);
    let test1 = properties.get("semicolon.no.quotes");
    let test2 = properties.get("semicolon.with.quotes");
    let test3 = properties.get("semicolon.with.spacing.no.quotes");
    let test4 = properties.get("semicolon.with.spacing.and.quotes");

    assert_eq!(test1, "pass");
    assert_eq!(test2, "pass");
    assert_eq!(test3, "pass");
    assert_eq!(test4, "pass");
}

#[test]
fn comments() {
    let properties: PropReader = PropReader::new(VALID_FILENAME);
    let test1 = properties.get("inline.comment.no.quotes");
    let test2 = properties.get("inline.comment.with.quotes");

    assert_eq!(test1, "pass");
    assert_eq!(test2, "pass");
}

#[test]
fn value_with_colon() {
    let properties: PropReader = PropReader::new(VALID_FILENAME);
    let test1 = properties.get("comment.with.multiple.colons.no.quotes");
    let test2 = properties.get("comment.with.multiple.colons.with.quotes");

    assert_eq!(test1, "pass:pass");
    assert_eq!(test2, "pass:pass");
}

#[test]
fn key_return() {
    let properties: PropReader = PropReader::new(SHORT_FILENAME);
    let test = properties.get_all_keys();
    assert!(test.contains(&"key".to_string()));
    assert!(test.contains(&"anotherkey".to_string()));
}

#[test]
fn value_return() {
    let properties: PropReader = PropReader::new(SHORT_FILENAME);
    let test = properties.get_all_values();
    assert!(test.contains(&"value".to_string()));
    assert!(test.contains(&"anothervalue".to_string()));
}

#[test]
fn large_file_load() {
    let properties: PropReader = PropReader::new(LARGE_FILENAME);
    let test = properties.get("key524745");
    assert_eq!(test, "value524745");
}

#[test]
#[should_panic]
fn read_invalid_properties() {
    let properties: PropReader = PropReader::new(INVALID_FILENAME);
    let test = properties.get("test2");
    println!("test");
    assert_eq!(test, "Invalid properties file.");
}

#[test]
#[should_panic]
fn properties_does_not_exist() {
    let _properties: PropReader = PropReader::new("fake_file.properties");
}
