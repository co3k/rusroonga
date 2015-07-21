extern crate libc;

mod groonga;

use libc::funcs::c95::string;
use std::ffi::{CStr, CString};
use std::{mem, str};
use std::rc::Rc;
use std::result::Result;
use std::result::Result::{Ok, Err};

pub const SUCCESS: i32 = groonga::GRN_SUCCESS;
pub const END_OF_DATA: i32 = groonga::GRN_END_OF_DATA;
pub const UNKNOWN_ERROR: i32 = groonga::GRN_UNKNOWN_ERROR;
pub const OPERATION_NOT_PERMITTED: i32 = groonga::GRN_OPERATION_NOT_PERMITTED;
pub const NO_SUCH_FILE_OR_DIRECTORY: i32 = groonga::GRN_NO_SUCH_FILE_OR_DIRECTORY;
pub const NO_SUCH_PROCESS: i32 = groonga::GRN_NO_SUCH_PROCESS;
pub const INTERRUPTED_FUNCTION_CALL: i32 = groonga::GRN_INTERRUPTED_FUNCTION_CALL;
pub const INPUT_OUTPUT_ERROR: i32 = groonga::GRN_INPUT_OUTPUT_ERROR;
pub const NO_SUCH_DEVICE_OR_ADDRESS: i32 = groonga::GRN_NO_SUCH_DEVICE_OR_ADDRESS;
pub const ARG_LIST_TOO_LONG: i32 = groonga::GRN_ARG_LIST_TOO_LONG;
pub const EXEC_FORMAT_ERROR: i32 = groonga::GRN_EXEC_FORMAT_ERROR;
pub const BAD_FILE_DESCRIPTOR: i32 = groonga::GRN_BAD_FILE_DESCRIPTOR;
pub const NO_CHILD_PROCESSES: i32 = groonga::GRN_NO_CHILD_PROCESSES;
pub const RESOURCE_TEMPORARILY_UNAVAILABLE: i32 = groonga::GRN_RESOURCE_TEMPORARILY_UNAVAILABLE;
pub const NOT_ENOUGH_SPACE: i32 = groonga::GRN_NOT_ENOUGH_SPACE;
pub const PERMISSION_DENIED: i32 = groonga::GRN_PERMISSION_DENIED;
pub const BAD_ADDRESS: i32 = groonga::GRN_BAD_ADDRESS;
pub const RESOURCE_BUSY: i32 = groonga::GRN_RESOURCE_BUSY;
pub const FILE_EXISTS: i32 = groonga::GRN_FILE_EXISTS;
pub const IMPROPER_LINK: i32 = groonga::GRN_IMPROPER_LINK;
pub const NO_SUCH_DEVICE: i32 = groonga::GRN_NO_SUCH_DEVICE;
pub const NOT_A_DIRECTORY: i32 = groonga::GRN_NOT_A_DIRECTORY;
pub const IS_A_DIRECTORY: i32 = groonga::GRN_IS_A_DIRECTORY;
pub const INVALID_ARGUMENT: i32 = groonga::GRN_INVALID_ARGUMENT;
pub const TOO_MANY_OPEN_FILES_IN_SYSTEM: i32 = groonga::GRN_TOO_MANY_OPEN_FILES_IN_SYSTEM;
pub const TOO_MANY_OPEN_FILES: i32 = groonga::GRN_TOO_MANY_OPEN_FILES;
pub const INAPPROPRIATE_I_O_CONTROL_OPERATION: i32 = groonga::GRN_INAPPROPRIATE_I_O_CONTROL_OPERATION;
pub const FILE_TOO_LARGE: i32 = groonga::GRN_FILE_TOO_LARGE;
pub const NO_SPACE_LEFT_ON_DEVICE: i32 = groonga::GRN_NO_SPACE_LEFT_ON_DEVICE;
pub const INVALID_SEEK: i32 = groonga::GRN_INVALID_SEEK;
pub const READ_ONLY_FILE_SYSTEM: i32 = groonga::GRN_READ_ONLY_FILE_SYSTEM;
pub const TOO_MANY_LINKS: i32 = groonga::GRN_TOO_MANY_LINKS;
pub const BROKEN_PIPE: i32 = groonga::GRN_BROKEN_PIPE;
pub const DOMAIN_ERROR: i32 = groonga::GRN_DOMAIN_ERROR;
pub const RESULT_TOO_LARGE: i32 = groonga::GRN_RESULT_TOO_LARGE;
pub const RESOURCE_DEADLOCK_AVOIDED: i32 = groonga::GRN_RESOURCE_DEADLOCK_AVOIDED;
pub const NO_MEMORY_AVAILABLE: i32 = groonga::GRN_NO_MEMORY_AVAILABLE;
pub const FILENAME_TOO_LONG: i32 = groonga::GRN_FILENAME_TOO_LONG;
pub const NO_LOCKS_AVAILABLE: i32 = groonga::GRN_NO_LOCKS_AVAILABLE;
pub const FUNCTION_NOT_IMPLEMENTED: i32 = groonga::GRN_FUNCTION_NOT_IMPLEMENTED;
pub const DIRECTORY_NOT_EMPTY: i32 = groonga::GRN_DIRECTORY_NOT_EMPTY;
pub const ILLEGAL_BYTE_SEQUENCE: i32 = groonga::GRN_ILLEGAL_BYTE_SEQUENCE;
pub const SOCKET_NOT_INITIALIZED: i32 = groonga::GRN_SOCKET_NOT_INITIALIZED;
pub const OPERATION_WOULD_BLOCK: i32 = groonga::GRN_OPERATION_WOULD_BLOCK;
pub const ADDRESS_IS_NOT_AVAILABLE: i32 = groonga::GRN_ADDRESS_IS_NOT_AVAILABLE;
pub const NETWORK_IS_DOWN: i32 = groonga::GRN_NETWORK_IS_DOWN;
pub const NO_BUFFER: i32 = groonga::GRN_NO_BUFFER;
pub const SOCKET_IS_ALREADY_CONNECTED: i32 = groonga::GRN_SOCKET_IS_ALREADY_CONNECTED;
pub const SOCKET_IS_NOT_CONNECTED: i32 = groonga::GRN_SOCKET_IS_NOT_CONNECTED;
pub const SOCKET_IS_ALREADY_SHUTDOWNED: i32 = groonga::GRN_SOCKET_IS_ALREADY_SHUTDOWNED;
pub const OPERATION_TIMEOUT: i32 = groonga::GRN_OPERATION_TIMEOUT;
pub const CONNECTION_REFUSED: i32 = groonga::GRN_CONNECTION_REFUSED;
pub const RANGE_ERROR: i32 = groonga::GRN_RANGE_ERROR;
pub const TOKENIZER_ERROR: i32 = groonga::GRN_TOKENIZER_ERROR;
pub const FILE_CORRUPT: i32 = groonga::GRN_FILE_CORRUPT;
pub const INVALID_FORMAT: i32 = groonga::GRN_INVALID_FORMAT;
pub const OBJECT_CORRUPT: i32 = groonga::GRN_OBJECT_CORRUPT;
pub const TOO_MANY_SYMBOLIC_LINKS: i32 = groonga::GRN_TOO_MANY_SYMBOLIC_LINKS;
pub const NOT_SOCKET: i32 = groonga::GRN_NOT_SOCKET;
pub const OPERATION_NOT_SUPPORTED: i32 = groonga::GRN_OPERATION_NOT_SUPPORTED;
pub const ADDRESS_IS_IN_USE: i32 = groonga::GRN_ADDRESS_IS_IN_USE;
pub const ZLIB_ERROR: i32 = groonga::GRN_ZLIB_ERROR;
pub const LZ4_ERROR: i32 = groonga::GRN_LZ4_ERROR;
pub const STACK_OVER_FLOW: i32 = groonga::GRN_STACK_OVER_FLOW;
pub const SYNTAX_ERROR: i32 = groonga::GRN_SYNTAX_ERROR;
pub const RETRY_MAX: i32 = groonga::GRN_RETRY_MAX;
pub const INCOMPATIBLE_FILE_FORMAT: i32 = groonga::GRN_INCOMPATIBLE_FILE_FORMAT;
pub const UPDATE_NOT_ALLOWED: i32 = groonga::GRN_UPDATE_NOT_ALLOWED;
pub const TOO_SMALL_OFFSET: i32 = groonga::GRN_TOO_SMALL_OFFSET;
pub const TOO_LARGE_OFFSET: i32 = groonga::GRN_TOO_LARGE_OFFSET;
pub const TOO_SMALL_LIMIT: i32 = groonga::GRN_TOO_SMALL_LIMIT;
pub const CAS_ERROR: i32 = groonga::GRN_CAS_ERROR;
pub const UNSUPPORTED_COMMAND_VERSION: i32 = groonga::GRN_UNSUPPORTED_COMMAND_VERSION;
pub const NORMALIZER_ERROR: i32 = groonga::GRN_NORMALIZER_ERROR;
pub const TOKEN_FILTER_ERROR: i32 = groonga::GRN_TOKEN_FILTER_ERROR;
pub const COMMAND_ERROR: i32 = groonga::GRN_COMMAND_ERROR;
pub const PLUGIN_ERROR: i32 = groonga::GRN_PLUGIN_ERROR;
pub const SCORER_ERROR: i32 = groonga::GRN_SCORER_ERROR;


pub const OBJ_TABLE_TYPE_MASK:        u16 = groonga::GRN_OBJ_TABLE_TYPE_MASK as u16;
pub const OBJ_TABLE_HASH_KEY:         u16 = groonga::GRN_OBJ_TABLE_HASH_KEY as u16;
pub const OBJ_TABLE_PAT_KEY:          u16 = groonga::GRN_OBJ_TABLE_PAT_KEY as u16;
pub const OBJ_TABLE_DAT_KEY:          u16 = groonga::GRN_OBJ_TABLE_DAT_KEY as u16;
pub const OBJ_TABLE_NO_KEY:           u16 = groonga::GRN_OBJ_TABLE_NO_KEY as u16;

pub const OBJ_KEY_MASK:               u16 = groonga::GRN_OBJ_KEY_MASK as u16;
pub const OBJ_KEY_UINT:               u16 = groonga::GRN_OBJ_KEY_UINT as u16;
pub const OBJ_KEY_INT:                u16 = groonga::GRN_OBJ_KEY_INT as u16;
pub const OBJ_KEY_FLOAT:              u16 = groonga::GRN_OBJ_KEY_FLOAT as u16;
pub const OBJ_KEY_GEO_POINT:          u16 = groonga::GRN_OBJ_KEY_GEO_POINT as u16;

pub const OBJ_KEY_WITH_SIS:           u16 = groonga::GRN_OBJ_KEY_WITH_SIS as u16;
pub const OBJ_KEY_NORMALIZE:          u16 = groonga::GRN_OBJ_KEY_NORMALIZE as u16;

pub const OBJ_COLUMN_TYPE_MASK:       u16 = groonga::GRN_OBJ_COLUMN_TYPE_MASK as u16;
pub const OBJ_COLUMN_SCALAR:          u16 = groonga::GRN_OBJ_COLUMN_SCALAR as u16;
pub const OBJ_COLUMN_VECTOR:          u16 = groonga::GRN_OBJ_COLUMN_VECTOR as u16;
pub const OBJ_COLUMN_INDEX:           u16 = groonga::GRN_OBJ_COLUMN_INDEX as u16;

pub const OBJ_COMPRESS_MASK:          u16 = groonga::GRN_OBJ_COMPRESS_MASK as u16;
pub const OBJ_COMPRESS_NONE:          u16 = groonga::GRN_OBJ_COMPRESS_NONE as u16;
pub const OBJ_COMPRESS_ZLIB:          u16 = groonga::GRN_OBJ_COMPRESS_ZLIB as u16;
pub const OBJ_COMPRESS_LZ4:           u16 = groonga::GRN_OBJ_COMPRESS_LZ4 as u16;

pub const OBJ_WITH_SECTION:           u16 = groonga::GRN_OBJ_WITH_SECTION as u16;
pub const OBJ_WITH_WEIGHT:            u16 = groonga::GRN_OBJ_WITH_WEIGHT as u16;
pub const OBJ_WITH_POSITION:          u16 = groonga::GRN_OBJ_WITH_POSITION as u16;
pub const OBJ_RING_BUFFER:            u16 = groonga::GRN_OBJ_RING_BUFFER as u16;

pub const OBJ_UNIT_MASK:              u16 = groonga::GRN_OBJ_UNIT_MASK as u16;
pub const OBJ_UNIT_DOCUMENT_NONE:     u16 = groonga::GRN_OBJ_UNIT_DOCUMENT_NONE as u16;
pub const OBJ_UNIT_DOCUMENT_SECTION:  u16 = groonga::GRN_OBJ_UNIT_DOCUMENT_SECTION as u16;
pub const OBJ_UNIT_DOCUMENT_POSITION: u16 = groonga::GRN_OBJ_UNIT_DOCUMENT_POSITION as u16;
pub const OBJ_UNIT_SECTION_NONE:      u16 = groonga::GRN_OBJ_UNIT_SECTION_NONE as u16;
pub const OBJ_UNIT_SECTION_POSITION:  u16 = groonga::GRN_OBJ_UNIT_SECTION_POSITION as u16;
pub const OBJ_UNIT_POSITION_NONE:     u16 = groonga::GRN_OBJ_UNIT_POSITION_NONE as u16;
pub const OBJ_UNIT_USERDEF_DOCUMENT:  u16 = groonga::GRN_OBJ_UNIT_USERDEF_DOCUMENT as u16;
pub const OBJ_UNIT_USERDEF_SECTION:   u16 = groonga::GRN_OBJ_UNIT_USERDEF_SECTION as u16;
pub const OBJ_UNIT_USERDEF_POSITION:  u16 = groonga::GRN_OBJ_UNIT_USERDEF_POSITION as u16;

pub const OBJ_NO_SUBREC:              u16 = groonga::GRN_OBJ_NO_SUBREC as u16;
pub const OBJ_WITH_SUBREC:            u16 = groonga::GRN_OBJ_WITH_SUBREC as u16;

pub const OBJ_KEY_VAR_SIZE:           u16 = groonga::GRN_OBJ_KEY_VAR_SIZE as u16;

pub const OBJ_TEMPORARY:              u16 = groonga::GRN_OBJ_TEMPORARY as u16;
pub const OBJ_PERSISTENT:             u16 = groonga::GRN_OBJ_PERSISTENT as u16;

pub const DB_VOID: u32 = groonga::GRN_DB_VOID;
pub const DB_DB: u32 = groonga::GRN_DB_DB;
pub const DB_OBJECT: u32 = groonga::GRN_DB_OBJECT;
pub const DB_BOOL: u32 = groonga::GRN_DB_BOOL;
pub const DB_INT8: u32 = groonga::GRN_DB_INT8;
pub const DB_UINT8: u32 = groonga::GRN_DB_UINT8;
pub const DB_INT16: u32 = groonga::GRN_DB_INT16;
pub const DB_UINT16: u32 = groonga::GRN_DB_UINT16;
pub const DB_INT32: u32 = groonga::GRN_DB_INT32;
pub const DB_UINT32: u32 = groonga::GRN_DB_UINT32;
pub const DB_INT64: u32 = groonga::GRN_DB_INT64;
pub const DB_UINT64: u32 = groonga::GRN_DB_UINT64;
pub const DB_FLOAT: u32 = groonga::GRN_DB_FLOAT;
pub const DB_TIME: u32 = groonga::GRN_DB_TIME;
pub const DB_SHORT_TEXT: u32 = groonga::GRN_DB_SHORT_TEXT;
pub const DB_TEXT: u32 = groonga::GRN_DB_TEXT;
pub const DB_LONG_TEXT: u32 = groonga::GRN_DB_LONG_TEXT;
pub const DB_TOKYO_GEO_POINT: u32 = groonga::GRN_DB_TOKYO_GEO_POINT;
pub const DB_WGS84_GEO_POINT: u32 = groonga::GRN_DB_WGS84_GEO_POINT;

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

    pub fn get(context: Rc<Context>, name: &str) -> Option<Object> {
        let c_name = CString::new(name).unwrap().as_ptr();
        unsafe {
            let obj = groonga::grn_ctx_get(
                context.ctx, c_name, string::strlen(c_name) as i32);
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
            groonga::grn_obj_unlink(self.context.ctx, self.obj);
            self.obj = mem::zeroed()
        }
    }

    fn path(&self) -> Option<&str> {
        unsafe {
            if self.obj.is_null() {
                return None
            }
            let path = groonga::grn_obj_path(self.context.ctx, self.obj);
            if path.is_null() {
                return None
            }
            Some(str::from_utf8(CStr::from_ptr(path).to_bytes()).unwrap())
        }
    }

    fn name(&self) -> Option<&str> {
        unsafe {
            if self.obj.is_null() {
                return None
            }
            let length = groonga::grn_obj_name(self.context.ctx, self.obj, mem::zeroed(), 0);
            if length == 0 {
                return Some("")
            }
            let buf = libc::malloc(
                mem::size_of::<libc::c_char>() as u64 * length as u64
            ) as *mut libc::c_char;
            groonga::grn_obj_name(self.context.ctx, self.obj, buf, length);
            let name = str::from_utf8(CStr::from_ptr(buf).to_bytes()).unwrap();
            libc::free(buf as *mut libc::c_void);
            Some(name)
        }
    }

    fn remove(&mut self) -> Result<(), Error> {
        unsafe {
            if self.obj.is_null() {
                return Err(Error::new(groonga::GRN_INVALID_ARGUMENT))
            }
            let rc = groonga::grn_obj_remove(self.context.ctx, self.obj);
            if rc != groonga::GRN_SUCCESS {
                return Err(Error::new(rc))
            }
            self.obj = mem::zeroed()
        }
        Ok(())
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
        if self.db.is_null() {
            return None
        }
        obj_path(self.context.clone(), self.db)
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

pub struct Table {
    object: Object
}

impl Table {
    fn from_object(object: Object) -> Table {
        Table{ object: object }
    }

    pub fn create(context: Rc<Context>, name: &str, path: Option<&str>, flags: u16, key_type: &Object, value_type: Option<&Object>) -> Result<Table, Error> {
        unsafe {
            let c_name = CString::new(name).unwrap().as_ptr();
            let c_path = match path {
                Some(p) => CString::new(p).unwrap().as_ptr(),
                None => mem::zeroed()
            };
            let c_value_type = match value_type {
                Some(vt) => vt.obj,
                None => mem::zeroed()
            };
            let tbl = groonga::grn_table_create(
                context.ctx, c_name, string::strlen(c_name) as u32,
                c_path, flags, key_type.obj, c_value_type);
            if tbl.is_null() {
                return Err(Error::new((*context.ctx).rc))
            }
            Ok(Table::from_object(Object{ context: context, obj: tbl }))
        }
    }

    pub fn open(context: Rc<Context>, name: &str) -> Option<Table> {
        match Context::get(context, name) {
            Some(object) => Some(Table::from_object(object)),
            None => None
        }
    }

    pub fn open_or_create(context: Rc<Context>, name: &str, path: Option<&str>, flags: u16, key_type: &Object, value_type: Option<&Object>) -> Result<Table, Error> {
        match Table::open(context.clone(), name) {
            None => Table::create(context, name, path, flags, key_type, value_type),
            Some(tbl) => Ok(tbl)
        }
    }

    pub fn name(&self) -> Option<&str> {
        self.object.name()
    }

    pub fn path(&self) -> Option<&str> {
        self.object.path()
    }

    pub fn remove(&mut self) -> Result<(), Error> {
        self.object.remove()
    }

    pub fn close(&mut self) {
        self.object.close()
    }
}

fn obj_path<'a>(context: Rc<Context>, obj: *mut groonga::grn_obj) -> Option<&'a str> {
    unsafe {
        let path = groonga::grn_obj_path(context.ctx, obj);
        if path.is_null() {
            return None
        }
        Some(str::from_utf8(CStr::from_ptr(path).to_bytes()).unwrap())
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
