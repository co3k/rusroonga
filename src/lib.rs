#![feature(libc)]
extern crate libc;

mod groonga;

use std::ffi::CString;
use std::fs;
use std::mem;
use std::result::Result;
use std::result::Result::{Ok, Err};

#[derive(Debug)]
pub struct Error {
    code: i32
}

impl Error {
    pub fn new(code: i32) -> Error {
        Error {code: code}
    }
}

pub struct Ctx {
    ctx: *mut groonga::grn_ctx,
    db: *mut groonga::grn_obj,
}

impl Drop for Ctx {
    fn drop(&mut self) {
        unsafe {
            if !self.db.is_null() {
                groonga::grn_obj_unlink(self.ctx, self.db);
            }

            let rc2 = groonga::grn_ctx_fin(self.ctx);
            if rc2 != groonga::GRN_SUCCESS {
                panic!("grn_ctx_fin(ctx) failed with rc2={}", rc2);
            }

            let rc3 = groonga::grn_fin();
            if rc3 != groonga::GRN_SUCCESS {
                panic!("grn_fin() failed with rc3={}", rc3);
            }
        }
    }
}

impl Ctx {
    pub fn new() -> Result<Ctx, Error> {
        unsafe {
            let rc = groonga::grn_init();
            if rc != groonga::GRN_SUCCESS {
                return Err(Error::new(rc))
            }

            let ctx = groonga::grn_ctx_open(0);
            if ctx.is_null() {
                return Err(Error::new(groonga::GRN_NO_MEMORY_AVAILABLE))
            }
            Ok(Ctx { ctx: ctx, db: mem::zeroed() })
        }
    }

    pub fn db_create(&mut self, path: &str) -> Result<(), Error> {
        let c_path = CString::new(path).unwrap();
        unsafe {
            self.db = groonga::grn_db_create(
                self.ctx, c_path.as_ptr(), mem::zeroed());
            if self.db.is_null() {
                return Err(Error::new((*self.ctx).rc))
            }
            Ok(())
        }
    }

    pub fn db_open(&mut self, path: &str) -> Result<(), Error> {
        let c_path = CString::new(path).unwrap();
        unsafe {
            self.db = groonga::grn_db_open(self.ctx, c_path.as_ptr());
            if self.db.is_null() {
                return Err(Error::new((*self.ctx).rc))
            }
            Ok(())
        }
    }

    pub fn db_open_or_create(&mut self, path: &str) -> Result<(), Error> {
        if file_exists(path) {
            self.db_open(path)
        } else {
            self.db_create(path)
        }
    }
}

fn file_exists(path: &str) -> bool {
    match fs::File::open(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}
