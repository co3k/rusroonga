extern crate rusroonga;
use rusroonga::groonga;

fn main() {
    let _ = rusroonga::Groonga::new().unwrap();
    let mut ctx = rusroonga::Context::new().unwrap();
println!("before open database");
    let mut db = ctx.db_open_or_create("test.db").unwrap();
println!("after open database");

    let short_text_type = ctx.ctx_at(groonga::GRN_DB_SHORT_TEXT).unwrap();
println!("after get short_text_type");
    let text_type = ctx.ctx_at(groonga::GRN_DB_TEXT).unwrap();
println!("after get text_type");
    let time_type = ctx.ctx_at(groonga::GRN_DB_TIME).unwrap();
println!("after get time_type");

println!("before open table");
    let table = ctx.table_open_or_create("Articles", "",
        groonga::GRN_OBJ_TABLE_HASH_KEY | groonga::GRN_OBJ_PERSISTENT,
        &short_text_type, &rusroonga::Obj::default()).unwrap();
println!("before open column title");
    let _ = ctx.column_open_or_create(&table, "title", "", 
        groonga::GRN_OBJ_PERSISTENT | groonga::GRN_OBJ_COLUMN_SCALAR,
        &short_text_type);
println!("before open column content");
    let _ = ctx.column_open_or_create(&table, "content", "", 
        groonga::GRN_OBJ_PERSISTENT | groonga::GRN_OBJ_COLUMN_SCALAR,
        &text_type);
println!("before open column updated_at");
    let _ = ctx.column_open_or_create(&table, "updated_at", "", 
        groonga::GRN_OBJ_PERSISTENT | groonga::GRN_OBJ_COLUMN_SCALAR,
        &time_type);
println!("after open column updated_at");
    db.close();
}
