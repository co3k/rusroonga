extern crate libc;

pub mod groonga;

//use libc::funcs::c95::string;
use std::ffi::{CStr, CString};
use std::{mem, str};
use std::rc::Rc;
use std::result::Result;
use std::result::Result::{Ok, Err};

#[derive(Debug)]
pub struct Error {
    pub code: i32
}

impl Error {
    pub fn new(code: i32) -> Error {
        Error {code: code}
    }
}

pub struct Groonga {
    closed: bool
}

impl Groonga {
    pub fn new() -> Result<Groonga, Error> {
        unsafe {
            let rc =  groonga::grn_init();
            if rc != groonga::GRN_SUCCESS {
                return Err(Error::new(rc))
            }
            Ok(Groonga{ closed: false })
        }
    }

    fn close(&mut self) -> Result<(), Error> {
        if self.closed {
            return Ok(())
        }
        unsafe {
            let rc = groonga::grn_fin();
            if rc != groonga::GRN_SUCCESS {
                return Err(Error::new(rc))
            }
            self.closed = true;
            Ok(())
        }
    }
}

impl Drop for Groonga {
    fn drop(&mut self) {
        self.close().unwrap()
    }
}

pub struct Context {
    ctx: *mut groonga::grn_ctx,
}

impl Drop for Context {
    fn drop(&mut self) {
        self.close().unwrap()
    }
}

impl Context {
    pub fn new() -> Result<Context, Error> {
        unsafe {
            let ctx = groonga::grn_ctx_open(0);
            if ctx.is_null() {
                return Err(Error::new(groonga::GRN_NO_MEMORY_AVAILABLE))
            }
            Ok(Context { ctx: ctx })
        }
    }

    pub fn at(context: Rc<Context>, id: u32) -> Option<Object> {
        unsafe {
            let obj = groonga::grn_ctx_at(context.ctx, id);
            if obj.is_null() {
                return None
            }
            Some(Object { context: context, obj: obj })
        }
    }

    pub fn close(&mut self) -> Result<(), Error> {
        unsafe {
            if self.ctx.is_null() {
                return Ok(())
            }
            let rc = groonga::grn_ctx_fin(self.ctx);
            if rc != groonga::GRN_SUCCESS {
                return Err(Error::new(rc))
            }
            self.ctx = mem::zeroed();
            Ok(())
        }
    }

//    pub fn ctx_get(&mut self, name: &str) -> Result<Object, Error> {
//        let c_name = CString::new(name).unwrap().as_ptr();
//        unsafe {
//            let obj = groonga::grn_ctx_get(
//                self.ctx, c_name, string::strlen(c_name) as i32);
//            if obj.is_null() {
//                return Err(Error::new((*self.ctx).rc))
//            }
//            Ok(Object { context: self, obj: obj })
//        }
//    }
//
//    pub fn table_create(&mut self, name: &str, path: &str, flags: u32,
//        key_type: &Object, value_type: Option<&Object>) -> Result<Object, Error> {
//        let c_name = CString::new(name).unwrap().as_ptr();
//        let c_path = if path != "" {
//            CString::new(path).unwrap().as_ptr()
//        } else {
//            ptr::null()
//        };
//        unsafe {
//            let c_value_type =
//                match value_type {
//                    Some(value_type_obj) => value_type_obj.obj,
//                    None => mem::zeroed(),
//                };
//            let table = groonga::grn_table_create(
//                self.ctx, c_name, string::strlen(c_name) as u32, c_path,
//                flags as u16, key_type.obj, c_value_type);
//            if table.is_null() {
//                return Err(Error::new((*self.ctx).rc))
//            }
//            Ok(Object { context: self, obj: table })
//        }
//    }
//
//    pub fn table_open_or_create(&mut self, name: &str, path: &str, flags: u32,
//        key_type: &Object, value_type: Option<&Object>) -> Result<Object, Error> {
//        if let Ok(table) = self.ctx_get(name) {
//            Ok(table)
//        } else {
//            self.table_create(name, path, flags,
//                key_type, value_type)
//        }
//    }
//
//    pub fn obj_column(&mut self, table: &Object, name: &str)
//        -> Result<Object, Error> {
//        let c_name = CString::new(name).unwrap().as_ptr();
//        unsafe {
//            let obj = groonga::grn_obj_column(
//                self.ctx, table.obj, c_name, string::strlen(c_name) as u32);
//            if obj.is_null() {
//                return Err(Error::new((*self.ctx).rc))
//            }
//            Ok(Object { context: self, obj: obj })
//        }
//    }
//
//    pub fn column_create(&mut self, table: &Object, name: &str, path: &str,
//        flags: u32, _type: &Object) -> Result<Object, Error> {
//        let c_name = CString::new(name).unwrap().as_ptr();
//        let c_path = if path != "" {
//            CString::new(path).unwrap().as_ptr()
//        } else {
//            ptr::null()
//        };
//        unsafe {
//            let column = groonga::grn_column_create(
//                self.ctx, table.obj, c_name, string::strlen(c_name) as u32,
//                c_path, flags as u16, _type.obj);
//            if column.is_null() {
//                return Err(Error::new((*self.ctx).rc))
//            }
//            Ok(Object { context: self, obj: column })
//        }
//    }
//
//    pub fn column_open_or_create(&mut self, table: &Object, name: &str,
//        path: &str, flags: u32, _type: &Object) -> Result<Object, Error> {
//        if let Ok(column) = self.obj_column(&table, name) {
//            Ok(column)
//        } else {
//            self.column_create(&table, name, path, flags, _type)
//        }
//    }
}

pub struct Object {
    context: Rc<Context>,
    obj: *mut groonga::grn_obj,
}

impl Drop for Object {
    fn drop(&mut self) {
        self.close()
    }
}

impl Object {
    pub fn close(&mut self) {
        unsafe {
            if self.obj.is_null() {
                return
            }
            close_obj(self.context.clone(), self.obj);
            self.obj = mem::zeroed()
        }
    }
}

pub struct Database {
    context: Rc<Context>,
    db: *mut groonga::grn_obj,
}

impl Database {
    pub fn create(context: Rc<Context>, path: &str) -> Result<Database, Error> {
        let c_path = CString::new(path).unwrap();
        unsafe {
            let db = groonga::grn_db_create(
                context.ctx, c_path.as_ptr(), mem::zeroed());
            if db.is_null() {
                return Err(Error::new((*context.ctx).rc))
            }
            Ok(Database{ context: context, db: db })
        }
    }

    pub fn open(context: Rc<Context>, path: &str) -> Result<Database, Error> {
        let c_path = CString::new(path).unwrap();
        unsafe {
            let db = groonga::grn_db_open(context.ctx, c_path.as_ptr());
            if db.is_null() {
                return Err(Error::new((*context.ctx).rc))
            }
            Ok(Database{ context: context, db: db })
        }
    }

    pub fn open_or_create(context: Rc<Context>, path: &str) -> Result<Database, Error> {
        match Database::open(context.clone(), path) {
            Err(e) => {
                if e.code == groonga::GRN_NO_SUCH_FILE_OR_DIRECTORY {
                    return Database::create(context.clone(), path)
                }
                Err(e)
            },
            Ok(db) => Ok(db)
        }
    }

    pub fn path(&self) -> Option<&str> {
        unsafe {
            if self.db.is_null() {
                return None
            }
            let path = groonga::grn_obj_path(self.context.ctx, self.db);
            if path.is_null() {
                return None
            }
            return Some(str::from_utf8(CStr::from_ptr(path).to_bytes()).unwrap())
        }
    }

    pub fn remove(&mut self) -> Result<(), Error> {
        unsafe {
            if self.db.is_null() {
                return Err(Error::new(groonga::GRN_INVALID_ARGUMENT))
            }
            let rv = remove_obj(self.context.clone(), self.db);
            self.db = mem::zeroed();
            rv
        }
    }

    pub fn close(&mut self) {
        unsafe {
            if self.db.is_null() {
                return
            }
            close_obj(self.context.clone(), self.db);
            self.db = mem::zeroed()
        }
    }
}

fn remove_obj(context: Rc<Context>, obj: *mut groonga::grn_obj) -> Result<(), Error> {
    unsafe {
        let rc = groonga::grn_obj_remove(context.ctx, obj);
        if rc != groonga::GRN_SUCCESS {
            return Err(Error::new(rc))
        }
    }
    Ok(())
}

fn close_obj(context: Rc<Context>, obj: *mut groonga::grn_obj) {
    unsafe {
        groonga::grn_obj_unlink(context.ctx, obj);
    }
}
