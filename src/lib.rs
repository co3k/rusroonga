#![feature(libc)]
extern crate libc;

pub mod groonga;

use libc::funcs::c95::string;
use std::ffi::CString;
use std::fs;
use std::mem;
use std::ptr;
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

pub struct Obj {
    ctx: *mut groonga::grn_ctx,
    obj: *mut groonga::grn_obj,
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

    pub fn ctx_at(&mut self, id: u32) -> Result<Obj, Error> {
        unsafe {
            let obj = groonga::grn_ctx_at(self.ctx, id);
            if obj.is_null() {
                return Err(Error::new((*self.ctx).rc))
            }
            Ok(Obj::new(self.ctx, obj))
        }
    }

    pub fn ctx_get(&mut self, name: &str) -> Result<Obj, Error> {
        let c_name = CString::new(name).unwrap().as_ptr();
        unsafe {
            let obj = groonga::grn_ctx_get(
                self.ctx, c_name, string::strlen(c_name) as i32);
            if obj.is_null() {
                return Err(Error::new((*self.ctx).rc))
            }
            Ok(Obj::new(self.ctx, obj))
        }
    }

    pub fn table_create(&mut self, name: &str, path: &str, flags: u32,
        key_type: &Obj, value_type: &Obj) -> Result<Obj, Error> {
        let c_name = CString::new(name).unwrap().as_ptr();
        let c_path = if path != "" {
            CString::new(path).unwrap().as_ptr()
        } else {
            ptr::null()
        };
        unsafe {
            let table = groonga::grn_table_create(
                self.ctx, c_name, string::strlen(c_name) as u32, c_path,
                flags as u16, key_type.obj, value_type.obj);
            if table.is_null() {
                return Err(Error::new((*self.ctx).rc))
            }
            Ok(Obj::new(self.ctx, table))
        }
    }

    pub fn table_open_or_create(&mut self, name: &str, path: &str, flags: u32,
        key_type: &Obj, value_type: &Obj) -> Result<Obj, Error> {
        if let Ok(table) = self.ctx_get(name) {
            Ok(table)
        } else {
            self.table_create(name, path, flags,
                key_type, value_type)
        }
    }

    pub fn new_obj_default(&mut self) -> Obj {
        Obj { ctx: self.ctx, obj: unsafe { mem::zeroed() } }
    }
}

impl Obj {
    fn new(ctx: *mut groonga::grn_ctx, obj: *mut groonga::grn_obj) -> Obj {
        Obj { ctx: ctx, obj: obj }
    }

    pub fn obj_column(&mut self, name: &str)
        -> Result<Obj, Error> {
        let c_name = CString::new(name).unwrap().as_ptr();
        unsafe {
            let obj = groonga::grn_obj_column(
                self.ctx, self.obj, c_name, string::strlen(c_name) as u32);
            if obj.is_null() {
                return Err(Error::new((*self.ctx).rc))
            }
            Ok(Obj::new(self.ctx, obj))
        }
    }

    pub fn column_create(&mut self, name: &str, path: &str,
        flags: u32, _type: &Obj) -> Result<Obj, Error> {
        let c_name = CString::new(name).unwrap().as_ptr();
        let c_path = if path != "" {
            CString::new(path).unwrap().as_ptr()
        } else {
            ptr::null()
        };
        unsafe {
            let column = groonga::grn_column_create(
                self.ctx, self.obj, c_name, string::strlen(c_name) as u32,
                c_path, flags as u16, _type.obj);
            if column.is_null() {
                return Err(Error::new((*self.ctx).rc))
            }
            Ok(Obj::new(self.ctx, column))
        }
    }

    pub fn column_open_or_create(&mut self, name: &str,
        path: &str, flags: u32, _type: &Obj) -> Result<Obj, Error> {
        if let Ok(column) = self.obj_column(name) {
            Ok(column)
        } else {
            self.column_create(name, path, flags, _type)
        }
    }
}

impl Drop for Obj {
    fn drop(&mut self) {
        unsafe {
            if !self.ctx.is_null() && !self.obj.is_null() {
                groonga::grn_obj_unlink(self.ctx, self.obj);
            }
        }
    }
}

fn file_exists(path: &str) -> bool {
    match fs::File::open(path) {
        Ok(_) => true,
        Err(_) => false,
    }
}
