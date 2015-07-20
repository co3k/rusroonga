extern crate rusroonga;
extern crate tempdir;

use rusroonga::{Context, Database, Table};
use std::{env, fs};
use std::rc::Rc;

#[test]
fn test_open_non_existent_table() {
    rusroonga::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(Context::new().unwrap());
    Database::create(ctx.clone(), path).unwrap();

    let table_name = "Table1";
    let table = Table::open(ctx.clone(), table_name);
    assert!(table.is_none(), "table should not exist");
}

//#[test]
//fn test_create_database() {
//    rusroonga::Groonga::new().unwrap();
//
//    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
//    let mut buf = work_dir.into_path();
//    buf.push("test.db");
//    let path = buf.to_str().unwrap();
//
//    let ctx = Rc::new(rusroonga::Context::new().unwrap());
//    let db = Database::create(ctx.clone(), path).unwrap();
//    assert_eq!(path, db.path().unwrap());
//    assert!(fs::metadata(path).unwrap().is_file());
//}
//
//#[test]
//fn test_open_or_create_database() {
//    rusroonga::Groonga::new().unwrap();
//
//    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
//    let mut buf = work_dir.into_path();
//    buf.push("test.db");
//    let path = buf.to_str().unwrap();
//
//    let ctx = Rc::new(rusroonga::Context::new().unwrap());
//    let db = Database::open_or_create(ctx.clone(), path).unwrap();
//    assert_eq!(path, db.path().unwrap());
//    assert!(fs::metadata(path).unwrap().is_file());
//}
//
//#[test]
//fn test_open_database() {
//    rusroonga::Groonga::new().unwrap();
//
//    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
//    let mut buf = work_dir.into_path();
//    buf.push("test.db");
//    let path = buf.to_str().unwrap();
//
//    let ctx = Rc::new(rusroonga::Context::new().unwrap());
//    let mut db = Database::create(ctx.clone(), path).unwrap();
//    db.close();
//
//    db = Database::open(ctx.clone(), path).unwrap();
//    assert_eq!(path, db.path().unwrap());
//    assert!(fs::metadata(path).unwrap().is_file());
//}
//
//#[test]
//fn test_remove_database() {
//    rusroonga::Groonga::new().unwrap();
//
//    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
//    let mut buf = work_dir.into_path();
//    buf.push("test.db");
//    let path = buf.to_str().unwrap();
//
//    let ctx = Rc::new(rusroonga::Context::new().unwrap());
//    let mut db = Database::create(ctx.clone(), path).unwrap();
//    let rv = db.remove();
//    assert!(rv.is_ok(), "failed to remove database");
//    assert!(fs::metadata(path).is_err(), "db file should not exist")
//}
