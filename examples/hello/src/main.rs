extern crate rusroonga;

use rusroonga as grn;
use std::env;
use std::rc::Rc;

fn main() {
    grn::Groonga::new().unwrap();

    let mut buf = env::temp_dir();
    buf.push("test.db");
    let path = buf.to_str().unwrap();
    println!("database path={}", path);

    let ctx = Rc::new(grn::Context::new().unwrap());
    let db = grn::Database::open_or_create(ctx.clone(), path).unwrap();

    let table1_name = "Table1";
    let table1_path = db.path().unwrap().to_string() + &".Table1";
    let table1 = grn::Table::open_or_create(
        ctx.clone(), table1_name, Some(&table1_path),
        grn::OBJ_TABLE_HASH_KEY | grn::OBJ_PERSISTENT,
        &grn::Context::at(ctx.clone(), grn::DB_SHORT_TEXT).unwrap(),
        None).unwrap();

    let column1_name = "column1";
    let column1_path = table1_path + &".column1";
    let _column1 = grn::Column::open_or_create(
        ctx.clone(), &table1, column1_name, Some(&column1_path),
        grn::OBJ_PERSISTENT | grn::OBJ_COLUMN_SCALAR,
        &grn::Context::at(ctx.clone(), grn::DB_TEXT).unwrap()).unwrap();
}
