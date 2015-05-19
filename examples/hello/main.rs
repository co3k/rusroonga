extern crate rusroonga;

fn main() {
    let mut ctx = rusroonga::Ctx::new().unwrap();
    ctx.db_open_or_create("test.db").unwrap();
}
