extern crate rusroonga;
extern crate tempdir;

use rusroonga as grn;
use std::{env, fs};
use std::rc::Rc;

#[test]
fn test_open_non_existent_table() {
    grn::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(grn::Context::new().unwrap());
    let _db = grn::Database::create(ctx.clone(), path).unwrap();

    let table_name = "Table1";
    let table = grn::Table::open(ctx.clone(), table_name);
    assert!(table.is_none(), "table should not exist");
}

#[test]
fn test_create_table() {
    grn::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(grn::Context::new().unwrap());
    let db = grn::Database::create(ctx.clone(), path).unwrap();

    let table1_name = "Table1";
    let table1_path = db.path().unwrap().to_string() + &".Table1";
    let table1 = grn::Table::create(
        ctx.clone(), table1_name, Some(&table1_path),
        grn::OBJ_TABLE_HASH_KEY | grn::OBJ_PERSISTENT,
        &grn::Context::at(ctx.clone(), grn::DB_SHORT_TEXT).unwrap(),
        None).unwrap();
    assert_eq!(table1_name, table1.name().unwrap());
    assert_eq!(table1_path, table1.path().unwrap());
    assert!(fs::metadata(path).unwrap().is_file());
}

#[test]
fn test_open_or_create_table() {
    grn::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(grn::Context::new().unwrap());
    let db = grn::Database::open_or_create(ctx.clone(), path).unwrap();

    let table1_name = "Table1";
    let table1_path = db.path().unwrap().to_string() + &".Table1";
    let table1 = grn::Table::open_or_create(
        ctx.clone(), table1_name, Some(&table1_path),
        grn::OBJ_TABLE_HASH_KEY | grn::OBJ_PERSISTENT,
        &grn::Context::at(ctx.clone(), grn::DB_SHORT_TEXT).unwrap(),
        None).unwrap();
    assert_eq!(table1_name, table1.name().unwrap());
    assert_eq!(table1_path, table1.path().unwrap());
    assert!(fs::metadata(path).unwrap().is_file());
}

#[test]
fn test_open_table() {
    grn::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(grn::Context::new().unwrap());
    let db = grn::Database::create(ctx.clone(), path).unwrap();

    let table1_name = "Table1";
    let table1_path = db.path().unwrap().to_string() + &".Table1";
    let mut table1 = grn::Table::create(
        ctx.clone(), table1_name, Some(&table1_path),
        grn::OBJ_TABLE_HASH_KEY | grn::OBJ_PERSISTENT,
        &grn::Context::at(ctx.clone(), grn::DB_SHORT_TEXT).unwrap(),
        None).unwrap();
    table1.close();
    assert!(fs::metadata(&table1_path).unwrap().is_file());

    let table1 = grn::Table::open(ctx.clone(), table1_name).unwrap();
    assert_eq!(table1_name, table1.name().unwrap());
    assert_eq!(table1_path, table1.path().unwrap());
    assert!(fs::metadata(&table1_path).unwrap().is_file());
}

#[test]
fn test_remove_table() {
    grn::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(grn::Context::new().unwrap());
    let db = grn::Database::create(ctx.clone(), path).unwrap();

    let table1_name = "Table1";
    let table1_path = db.path().unwrap().to_string() + &".Table1";
    let mut table1 = grn::Table::create(
        ctx.clone(), table1_name, Some(&table1_path),
        grn::OBJ_TABLE_HASH_KEY | grn::OBJ_PERSISTENT,
        &grn::Context::at(ctx.clone(), grn::DB_SHORT_TEXT).unwrap(),
        None).unwrap();
    let rv = table1.remove();
    assert!(rv.is_ok(), "failed to remove table");
    assert!(fs::metadata(&table1_path).is_err(), "table file should not exist")
}

#[test]
fn test_add_record() {
    grn::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(grn::Context::new().unwrap());
    let db = grn::Database::create(ctx.clone(), path).unwrap();

    let table1_name = "Table1";
    let table1_path = db.path().unwrap().to_string() + &".Table1";
    let mut table1 = grn::Table::create(
        ctx.clone(), table1_name, Some(&table1_path),
        grn::OBJ_TABLE_HASH_KEY | grn::OBJ_PERSISTENT,
        &grn::Context::at(ctx.clone(), grn::DB_SHORT_TEXT).unwrap(),
        None).unwrap();

    let (id, added) = table1.add_record(Some("foo"));
    assert!(added, "record should be added");
    let (id2, added) = table1.add_record(Some("foo"));
    assert!(!added, "record should not be added because it already exists");
    assert_eq!(id, id2)
}

#[test]
fn test_lcp_search() {
    grn::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(grn::Context::new().unwrap());
    let db = grn::Database::create(ctx.clone(), path).unwrap();

    let table1_name = "Table1";
    let table1_path = db.path().unwrap().to_string() + &".Table1";
    let mut table1 = grn::Table::create(
        ctx.clone(), table1_name, Some(&table1_path),
        grn::OBJ_TABLE_HASH_KEY | grn::OBJ_PERSISTENT,
        &grn::Context::at(ctx.clone(), grn::DB_SHORT_TEXT).unwrap(),
        None).unwrap();

    let (id, added) = table1.add_record(Some("foo"));
    let id2 = table1.lcp_search(Some("foo"));

    assert_eq!(id, id2)
}

#[test]
fn test_pat_scan() {
    grn::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(grn::Context::new().unwrap());
    let db = grn::Database::create(ctx.clone(), path).unwrap();

    let table1_name = "Table1";
    let table1_path = db.path().unwrap().to_string() + &".Table1";
    let mut table1 = grn::Table::create(
        ctx.clone(), table1_name, Some(&table1_path),
        grn::OBJ_TABLE_PAT_KEY | grn::OBJ_PERSISTENT,
        &grn::Context::at(ctx.clone(), grn::DB_SHORT_TEXT).unwrap(),
        None).unwrap();

    table1.add_record(Some("foo"));
    let (nhit, hit, rest) = table1.pat_scan(Some("ebi foo kani foo sasori"));

    assert_eq!(nhit, 2);
    assert_eq!(hit.id, 1);
    assert_eq!(hit.offset, 4);
    assert_eq!(hit.length, 3);
    assert_eq!(rest, "");
}
