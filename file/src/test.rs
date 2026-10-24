use std::{
    thread::sleep,
    time::{Duration, SystemTime},
};

use crate::ans::ReadOnlyFile;

#[test]
fn basic_getter_test() {
    const FILE_CONTENT: &str = "Hello Test!";
    let file = ReadOnlyFile::from_str(FILE_CONTENT, "I am owner");
    assert_eq!(file.owner(), "I am owner");
    assert!(file.permitted_users().is_empty());

    assert!(file.add_permitted_user("I am owner", "I can read!").is_ok());
    assert!(file.permitted_users().contains("I can read!"));
    assert!(file
        .add_permitted_user("I am owner", "I can read too!")
        .is_ok());
    assert!(file.permitted_users().contains("I can read!"));
    assert!(file.permitted_users().contains("I can read too!"));

    assert!(file
        .remove_permitted_user("I am owner", "I can read too!")
        .is_ok());
    assert!(file.permitted_users().contains("I can read!"));
    assert!(!file.permitted_users().contains("I can read too!"));
}

#[test]
fn read_owner_test() {
    const FILE_CONTENT: &str = "Hello Test!";
    let file = ReadOnlyFile::from_str(FILE_CONTENT, "I am owner");
    assert!(file.read("I am owner").is_ok());
    assert!(file.read("I am owner?").is_err());
    assert!(file.read("I am not owner").is_err());
}

#[test]
fn read_compare_test() {
    const FILE_CONTENT: &str = "Hello Test!";
    let file = ReadOnlyFile::from_str(FILE_CONTENT, "I am owner");
    let content = file.read("I am owner").unwrap();
    assert_eq!(content, FILE_CONTENT.as_bytes());
}

#[test]
fn read_time_tset() {
    const FILE_CONTENT: &str = "Hello Test!";
    let file = ReadOnlyFile::from_str(FILE_CONTENT, "I am owner");

    sleep(Duration::from_millis(10));
    let time = SystemTime::now();
    sleep(Duration::from_millis(10));

    file.read("I am owner").unwrap();
    let last_read_time = file.latest_read_time();
    assert!(last_read_time > time);
}

#[test]
fn add_user_test() {
    const FILE_CONTENT: &str = "Hello Test!";
    let file = ReadOnlyFile::from_str(FILE_CONTENT, "I am owner");
    assert!(file.add_permitted_user("I am owner", "I can read!").is_ok());
    assert!(file.read("I can read!").is_ok());
    assert!(file.read("I can read?").is_err());

    assert!(file
        .add_permitted_user("I am not owner", "I can read?")
        .is_err());
}

#[test]
fn remove_user_test() {
    const FILE_CONTENT: &str = "Hello Test!";
    let file = ReadOnlyFile::from_str(FILE_CONTENT, "I am owner");
    assert!(file.add_permitted_user("I am owner", "I can read!").is_ok());
    assert!(file.add_permitted_user("I am owner", "I can read!").is_ok());
    assert!(file.add_permitted_user("I am owner", "I can read!").is_ok());
    assert!(file.read("I can read!").is_ok());

    assert!(file
        .remove_permitted_user("I am owner", "I am owner")
        .is_err());

    assert!(file
        .remove_permitted_user("I am owner", "I can read!")
        .is_ok());
    assert!(file.read("I can read!").is_err());

    assert!(file
        .remove_permitted_user("I am owner", "I can read!")
        .is_ok());
    assert!(file.read("I can read!").is_err());

    assert!(file.add_permitted_user("I am owner", "I can read!").is_ok());
    assert!(file.read("I can read!").is_ok());
}
