extern crate rusroonga;
use rusroonga::groonga;

fn main() {
    rusroonga::Groonga::new().unwrap();
    let mut ctx = rusroonga::Context::new().unwrap();
    ctx.db_open_or_create("test.db").unwrap();

    let short_text_type = ctx.ctx_at(groonga::GRN_DB_SHORT_TEXT).unwrap();
    let text_type = ctx.ctx_at(groonga::GRN_DB_TEXT).unwrap();
    let time_type = ctx.ctx_at(groonga::GRN_DB_TIME).unwrap();

    let table = ctx.table_open_or_create("Articles", "",
        groonga::GRN_OBJ_TABLE_HASH_KEY | groonga::GRN_OBJ_PERSISTENT,
        &short_text_type, &rusroonga::Obj::default()).unwrap();
    let _ = ctx.column_open_or_create(&table, "title", "", 
        groonga::GRN_OBJ_PERSISTENT | groonga::GRN_OBJ_COLUMN_SCALAR,
        &short_text_type);
    let _ = ctx.column_open_or_create(&table, "content", "", 
        groonga::GRN_OBJ_PERSISTENT | groonga::GRN_OBJ_COLUMN_SCALAR,
        &text_type);
    let _ = ctx.column_open_or_create(&table, "updated_at", "", 
        groonga::GRN_OBJ_PERSISTENT | groonga::GRN_OBJ_COLUMN_SCALAR,
        &time_type);
}
