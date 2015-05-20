extern crate rusroonga;
use rusroonga::groonga;

#[link(name="ruroonga", kind="static")]
fn main() {
    let mut ctx = rusroonga::Ctx::new().unwrap();
    ctx.db_open_or_create("test.db").unwrap();

    let short_text_type = ctx.ctx_at(groonga::GRN_DB_SHORT_TEXT).unwrap();
    let text_type = ctx.ctx_at(groonga::GRN_DB_TEXT).unwrap();
    let time_type = ctx.ctx_at(groonga::GRN_DB_TIME).unwrap();

    let null_obj = ctx.new_obj_default();
    let table = ctx.table_open_or_create("Articles", "",
        groonga::GRN_OBJ_TABLE_HASH_KEY | groonga::GRN_OBJ_PERSISTENT,
        &short_text_type, &null_obj).unwrap();
    let title_column = ctx.column_open_or_create(&table, "title", "", 
        groonga::GRN_OBJ_PERSISTENT | groonga::GRN_OBJ_COLUMN_SCALAR,
        &short_text_type).unwrap();
    let _ = ctx.column_open_or_create(&table, "content", "", 
        groonga::GRN_OBJ_PERSISTENT | groonga::GRN_OBJ_COLUMN_SCALAR,
        &text_type).unwrap();
    let _ = ctx.column_open_or_create(&table, "updated_at", "", 
        groonga::GRN_OBJ_PERSISTENT | groonga::GRN_OBJ_COLUMN_SCALAR,
        &time_type).unwrap();

    let (record_id, added) = ctx.table_add(&table,
        "http://ja.wikipedia.org/wiki/%E6%B7%B1%E6%B5%B7%E9%AD%9A");
    println!("record_id={}, added={}", record_id, added);
    let mut title_value = ctx.new_obj_default();
    title_value.text_init(0);
    ctx.text_put(&title_value, "深海魚");
    ctx.obj_set_value(&title_column, record_id, &title_value,
        groonga::GRN_OBJ_SET).unwrap();
}
