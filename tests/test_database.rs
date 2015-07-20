extern crate rusroonga;
extern crate tempdir;

use std::{env, fs};

#[test]
fn test_create_database() {
    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    rusroonga::Groonga::new().unwrap();
    let mut ctx = rusroonga::Context::new().unwrap();
    let db = ctx.db_create(path).unwrap();
    assert_eq!(path, db.path().unwrap());
    assert!(fs::metadata(path).unwrap().is_file());
}

#[test]
fn test_open_database() {
    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    rusroonga::Groonga::new().unwrap();
    let mut ctx = rusroonga::Context::new().unwrap();
    let mut db = ctx.db_create(path).unwrap();
    db.close();
    db = ctx.db_open(path).unwrap();
    assert_eq!(path, db.path().unwrap());
    assert!(fs::metadata(path).unwrap().is_file());
}

#[test]
fn test_remove_database() {
    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    rusroonga::Groonga::new().unwrap();
    let mut ctx = rusroonga::Context::new().unwrap();
    let mut db = ctx.db_create(path).unwrap();
    assert!(db.remove().is_ok(), "failed to remove database");
    assert!(fs::metadata(path).is_err(), "db file should not exist")
}
