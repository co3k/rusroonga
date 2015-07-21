extern crate rusroonga;
extern crate tempdir;

use rusroonga as grn;
use rusroonga::{Column, Context, Database, Table};
use std::{env, fs};
use std::rc::Rc;

#[test]
fn test_open_non_existent_column() {
    rusroonga::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(Context::new().unwrap());
    let _db = Database::create(ctx.clone(), path).unwrap();

    let table1_name = "Table1";
    let table1 = Table::create(
        ctx.clone(), table1_name, None,
        grn::OBJ_TABLE_HASH_KEY | grn::OBJ_PERSISTENT,
        &Context::at(ctx.clone(), grn::DB_SHORT_TEXT).unwrap(),
        None).unwrap();

    let column1_name = "Column1";
    let column1 = Column::open(ctx.clone(), &table1, column1_name);
    assert!(column1.is_none(), "column should not exist");
}

#[test]
fn test_create_column() {
    rusroonga::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(rusroonga::Context::new().unwrap());
    let db = Database::create(ctx.clone(), path).unwrap();

    let table1_name = "Table1";
    let table1_path = db.path().unwrap().to_string() + &".Table1";
    let table1 = Table::create(
        ctx.clone(), table1_name, Some(&table1_path),
        grn::OBJ_TABLE_HASH_KEY | grn::OBJ_PERSISTENT,
        &Context::at(ctx.clone(), grn::DB_SHORT_TEXT).unwrap(),
        None).unwrap();

    let column1_name = "column1";
    let column1_path = table1_path + &".column1";
    let column1 = Column::create(
        ctx.clone(), &table1, column1_name, Some(&column1_path),
        grn::OBJ_PERSISTENT | grn::OBJ_COLUMN_SCALAR,
        &Context::at(ctx.clone(), grn::DB_TEXT).unwrap()).unwrap();
    assert_eq!(
        table1_name.to_string() + &"." + &column1_name,
        column1.name().unwrap());
    assert_eq!(column1_path, column1.path().unwrap());
    assert!(fs::metadata(&column1_path).unwrap().is_file());
}

#[test]
fn test_open_or_create_column() {
    rusroonga::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(rusroonga::Context::new().unwrap());
    let db = Database::open_or_create(ctx.clone(), path).unwrap();

    let table1_name = "Table1";
    let table1_path = db.path().unwrap().to_string() + &".Table1";
    let table1 = Table::open_or_create(
        ctx.clone(), table1_name, Some(&table1_path),
        grn::OBJ_TABLE_HASH_KEY | grn::OBJ_PERSISTENT,
        &Context::at(ctx.clone(), grn::DB_SHORT_TEXT).unwrap(),
        None).unwrap();

    let column1_name = "column1";
    let column1_path = table1_path + &".column1";
    let column1 = Column::open_or_create(
        ctx.clone(), &table1, column1_name, Some(&column1_path),
        grn::OBJ_PERSISTENT | grn::OBJ_COLUMN_SCALAR,
        &Context::at(ctx.clone(), grn::DB_TEXT).unwrap()).unwrap();
    assert_eq!(
        table1_name.to_string() + &"." + &column1_name,
        column1.name().unwrap());
    assert_eq!(column1_path, column1.path().unwrap());
    assert!(fs::metadata(&column1_path).unwrap().is_file());
}

#[test]
fn test_open_column() {
    rusroonga::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(rusroonga::Context::new().unwrap());
    let db = Database::create(ctx.clone(), path).unwrap();

    let table1_name = "Table1";
    let table1_path = db.path().unwrap().to_string() + &".Table1";
    let table1 = Table::create(
        ctx.clone(), table1_name, Some(&table1_path),
        grn::OBJ_TABLE_HASH_KEY | grn::OBJ_PERSISTENT,
        &Context::at(ctx.clone(), grn::DB_SHORT_TEXT).unwrap(),
        None).unwrap();

    let column1_name = "column1";
    let column1_path = table1_path + &".column1";
    let mut column1 = Column::create(
        ctx.clone(), &table1, column1_name, Some(&column1_path),
        grn::OBJ_PERSISTENT | grn::OBJ_COLUMN_SCALAR,
        &Context::at(ctx.clone(), grn::DB_TEXT).unwrap()).unwrap();
    column1.close();

    let column1 = Column::open(ctx.clone(), &table1, column1_name).unwrap();
    assert_eq!(
        table1_name.to_string() + &"." + &column1_name,
        column1.name().unwrap());
    assert_eq!(column1_path, column1.path().unwrap());
    assert!(fs::metadata(&column1_path).unwrap().is_file());
}

#[test]
fn test_remove_column() {
    rusroonga::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(rusroonga::Context::new().unwrap());
    let db = Database::create(ctx.clone(), path).unwrap();

    let table1_name = "Table1";
    let table1_path = db.path().unwrap().to_string() + &".Table1";
    let table1 = Table::create(
        ctx.clone(), table1_name, Some(&table1_path),
        grn::OBJ_TABLE_HASH_KEY | grn::OBJ_PERSISTENT,
        &Context::at(ctx.clone(), grn::DB_SHORT_TEXT).unwrap(),
        None).unwrap();

    let column1_name = "column1";
    let column1_path = table1_path + &".column1";
    let mut column1 = Column::create(
        ctx.clone(), &table1, column1_name, Some(&column1_path),
        grn::OBJ_PERSISTENT | grn::OBJ_COLUMN_SCALAR,
        &Context::at(ctx.clone(), grn::DB_TEXT).unwrap()).unwrap();
    let rv = column1.remove();
    assert!(rv.is_ok(), "failed to remove column");
    assert!(fs::metadata(&column1_path).is_err(), "column file should not exist")
}
