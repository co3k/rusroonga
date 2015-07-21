//extern crate core;
extern crate libc;

mod groonga;

//use core::array::FixedSizeArray;
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

// obj_flags values
pub const OBJ_REFER            :libc::c_uchar = (0x01<<0);
pub const OBJ_OUTPLACE         :libc::c_uchar = (0x01<<1);
pub const OBJ_DO_SHALLOW_COPY  :libc::c_uchar = (OBJ_REFER|OBJ_OUTPLACE);
pub const OBJ_VECTOR           :libc::c_uchar = (0x01<<7);

// grn_obj_set_value flags values
pub const OBJ_SET_MASK  :libc::c_int = (0x07);
pub const OBJ_SET       :libc::c_int = (0x01);
pub const OBJ_INCR      :libc::c_int = (0x02);
pub const OBJ_DECR      :libc::c_int = (0x03);
pub const OBJ_APPEND    :libc::c_int = (0x04);
pub const OBJ_PREPEND   :libc::c_int = (0x05);
pub const OBJ_GET       :libc::c_int = (0x01<<4);
pub const OBJ_COMPARE   :libc::c_int = (0x01<<5);
pub const OBJ_LOCK      :libc::c_int = (0x01<<6);
pub const OBJ_UNLOCK    :libc::c_int = (0x01<<7);

#[inline]
fn obj_init(obj: *mut groonga::grn_obj, obj_type: libc::c_uchar, obj_flags: libc::c_uchar, obj_domain: libc::c_uint) {
    unsafe {
        (*obj).header._type = obj_type;
        (*obj).header.impl_flags = obj_flags;
        (*obj).header.flags = 0;
        (*obj).header.domain = obj_domain;
        let b = (*obj).u.b();
        (*b).head = mem::zeroed();
        (*b).curr = mem::zeroed();
        (*b).tail = mem::zeroed();
    }
}

#[inline]
fn value_fix_size_init(obj: *mut groonga::grn_obj, obj_flags: libc::c_uchar, obj_domain: libc::c_uint) {
    let obj_type = if obj_flags & OBJ_VECTOR != 0 {
        groonga::GRN_UVECTOR as u8
    } else {
        groonga::GRN_BULK as u8
    };
    obj_init(obj, obj_type, obj_flags & OBJ_DO_SHALLOW_COPY, obj_domain)
}

#[inline]
fn value_var_size_init(obj: *mut groonga::grn_obj, obj_flags: libc::c_uchar, obj_domain: libc::c_uint) {
    let obj_type = if obj_flags & OBJ_VECTOR != 0 {
        groonga::GRN_VECTOR as u8
    } else {
        groonga::GRN_BULK as u8
    };
    obj_init(obj, obj_type, obj_flags & OBJ_DO_SHALLOW_COPY, obj_domain)
}

#[inline]
fn text_init(obj: *mut groonga::grn_obj, obj_flags: libc::c_uchar) {
    value_var_size_init(obj, obj_flags, groonga::GRN_DB_TEXT)
}

#[inline]
fn text_put(ctx: *mut groonga::grn_ctx, bulk: *mut groonga::grn_obj,
             str: *const ::libc::c_char, len: ::libc::c_uint) -> groonga::grn_rc {
    unsafe {
        groonga::grn_bulk_write(ctx, bulk, str, len)
    }
}

#[inline]
fn bulk_outp(bulk: *const groonga::grn_obj) -> bool {
    unsafe {
        (*bulk).header.impl_flags & OBJ_OUTPLACE != 0
    }
}

/* This assumes that GRN_BULK_BUFSIZE is less than 32 (= 0x20). */
const BULK_BUFSIZE_MAX: u16 = 0x1f;

#[inline]
fn bulk_size_in_flags(flags: u16) -> u16 {
    flags & BULK_BUFSIZE_MAX
}

// NOTE: bulk is need to be mutable because of b(&mut self)
#[inline]
fn bulk_vsize(bulk: *mut groonga::grn_obj) -> usize {
    unsafe {
        if bulk_outp(bulk) {
            let b = (*bulk).u.b();
            (*b).curr as usize - (*b).head as usize
        } else {
            bulk_size_in_flags((*bulk).header.flags) as usize
        }
    }
}

#[inline]
fn bulk_head(bulk: *mut groonga::grn_obj) -> *mut libc::c_char {
    unsafe {
        if bulk_outp(bulk) {
            (*((*bulk).u.b())).head
        } else {
            let raw: *mut i8 = ::std::mem::transmute(&(*((*bulk).u.b())).head);
            ::std::mem::transmute(raw.offset(0))
        }
    }
}

#[inline]
fn text_len(obj: *mut groonga::grn_obj) -> usize {
    bulk_vsize(obj)
}

#[inline]
fn text_value(obj: *mut groonga::grn_obj) -> *mut libc::c_char {
    bulk_head(obj)
}

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
    fn new(context: Rc<Context>, obj: *mut groonga::grn_obj) -> Object {
        Object{context: context, obj: obj}
    }

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
    object: Object
}

impl Database {
    fn from_object(object: Object) -> Database {
        Database{ object: object }
    }

    pub fn create(context: Rc<Context>, path: &str) -> Result<Database, Error> {
        let c_path = CString::new(path).unwrap();
        unsafe {
            let db = groonga::grn_db_create(
                context.ctx, c_path.as_ptr(), mem::zeroed());
            if db.is_null() {
                return Err(Error::new((*context.ctx).rc))
            }
            Ok(Database::from_object(Object::new(context, db)))
        }
    }

    pub fn open(context: Rc<Context>, path: &str) -> Result<Database, Error> {
        let c_path = CString::new(path).unwrap();
        unsafe {
            let db = groonga::grn_db_open(context.ctx, c_path.as_ptr());
            if db.is_null() {
                return Err(Error::new((*context.ctx).rc))
            }
            Ok(Database::from_object(Object::new(context, db)))
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

pub type ID = groonga::grn_id;

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
            Ok(Table::from_object(Object::new(context, tbl)))
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

    pub fn add_record(&mut self, key: Option<&str>) -> (ID, bool) {
        unsafe {
            let c_key = match key {
                Some(k) => CString::new(k).unwrap().as_ptr(),
                None => mem::zeroed()
            };
            let mut c_added: libc::c_int = 0;
            let id = groonga::grn_table_add(
                self.object.context.ctx, self.object.obj,
                c_key as *const libc::c_void, string::strlen(c_key) as u32, &mut c_added);
            (id, c_added != 0)
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

pub struct Column {
    object: Object
}

impl Column {
    fn from_object(object: Object) -> Column {
        Column{ object: object }
    }

    pub fn create(context: Rc<Context>, table: &Table, name: &str, path: Option<&str>, flags: u16, column_type: &Object) -> Result<Column, Error> {
        unsafe {
            let c_name = CString::new(name).unwrap().as_ptr();
            let c_path = match path {
                Some(p) => CString::new(p).unwrap().as_ptr(),
                None => mem::zeroed()
            };
            let tbl = groonga::grn_column_create(
                context.ctx, table.object.obj,
                c_name, string::strlen(c_name) as u32,
                c_path, flags, column_type.obj);
            if tbl.is_null() {
                return Err(Error::new((*context.ctx).rc))
            }
            Ok(Column::from_object(Object::new(context, tbl)))
        }
    }

    pub fn open(context: Rc<Context>, table: &Table, name: &str) -> Option<Column> {
        let c_name = CString::new(name).unwrap().as_ptr();
        unsafe {
            let col = groonga::grn_obj_column(
                context.ctx, table.object.obj, c_name, string::strlen(c_name) as u32);
            if col.is_null() {
                return None
            }
            Some(Column::from_object(Object::new(context, col)))
        }
    }

    pub fn open_or_create(context: Rc<Context>, table: &Table, name: &str, path: Option<&str>, flags: u16, column_type: &Object) -> Result<Column, Error> {
        match Column::open(context.clone(), table, name) {
            None => Column::create(context, table, name, path, flags, column_type),
            Some(col) => Ok(col)
        }
    }

    pub fn set_string(&mut self, record_id: ID, s: Option<&str>) -> Result<(), Error> {
        unsafe {
            let c_s = match s {
                Some(s) => CString::new(s).unwrap().as_ptr(),
                None => mem::zeroed()
            };

            let mut buf = groonga::grn_obj::default();
            text_init(&mut buf, 0);
            text_put(self.object.context.ctx, &mut buf, c_s,
                     string::strlen(c_s) as u32);
            let rc = groonga::grn_obj_set_value(
                self.object.context.ctx, self.object.obj, record_id, &mut buf,
                OBJ_SET);
            if rc != groonga::GRN_SUCCESS {
                return Err(Error::new(rc))
            }
            Ok(())
        }
    }

    pub fn get_string(&self, record_id: ID) -> String {
        let mut buf = groonga::grn_obj::default();
        text_init(&mut buf, 0);
        unsafe {
            groonga::grn_obj_get_value(
                self.object.context.ctx, self.object.obj, record_id, &mut buf);
            let head = text_value(&mut buf) as *const u8;
            let size = text_len(&mut buf);
            str::from_utf8(std::slice::from_raw_parts(head, size)).unwrap().to_string()
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
