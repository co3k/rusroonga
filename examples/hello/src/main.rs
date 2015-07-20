extern crate rusroonga;
extern crate tempdir;

use rusroonga::Database;
use std::{env, fs};
use std::rc::Rc;

fn main() {
    let work_dir = tempdir::TempDir::new(env::temp_dir().to_str().unwrap()).unwrap();
    let mut buf = work_dir.into_path();
    buf.push("test.db");
    let path = buf.to_str().unwrap();

    rusroonga::Groonga::new().unwrap();
    let ctx = Rc::new(rusroonga::Context::new().unwrap());
    let db = Database::open_or_create(ctx.clone(), path).unwrap();
    assert_eq!(path, db.path().unwrap());
    assert!(fs::metadata(path).unwrap().is_file());
}
