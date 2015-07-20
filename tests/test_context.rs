extern crate rusroonga;
extern crate tempdir;

use rusroonga::{Context, Database};
use std::env;
use std::rc::Rc;

#[test]
fn test_context_at() {
    rusroonga::Groonga::new().unwrap();

    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    let ctx = Rc::new(Context::new().unwrap());
    Database::create(ctx.clone(), path).unwrap();

    let obj = Context::at(ctx.clone(), rusroonga::DB_SHORT_TEXT);
    assert!(obj.is_some());
}
