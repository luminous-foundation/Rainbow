use libloading::{Library, Symbol};
use libffi::{low::*, raw::ffi_abi_FFI_GNUW64};
use std::{ffi::c_void, ptr::addr_of_mut};
use crate::{_type::{Type, Types}, ffi, frame::Frame, function::Extern, value::{Value, Values}};

pub unsafe fn type_to_type(typ: &Type) -> ffi_type {
    match typ.typ[0] {
        Types::VOID    => types::void,
        Types::I8      => types::sint8,
        Types::I16     => types::sint16,
        Types::I32     => types::sint32,
        Types::I64     => types::sint64,
        Types::U8      => types::uint8,
        Types::U16     => types::uint16,
        Types::U32     => types::uint32,
        Types::U64     => types::uint64,
        Types::F32     => types::float,
        Types::F64     => types::double,
        Types::POINTER => types::pointer,
        _ => panic!("unsupported type {:?} for externs", typ.typ[0])
    }
}

pub unsafe fn get_pointer(val: &Value, sp: &mut Vec<i64>, up: &mut Vec<u64>, dp: &mut Vec<f64>, pp: &mut Vec<*mut c_void>) -> *mut c_void {
    match val.val {
        Values::SIGNED(n) => {
            sp.push(n);
            pp.push(sp.last_mut().unwrap() as *mut _ as *mut c_void);
            pp.last_mut().unwrap() as *mut _ as *mut c_void
        }
        Values::UNSIGNED(n) => {
            up.push(n);
            pp.push(up.last_mut().unwrap() as *mut _ as *mut c_void);
            pp.last_mut().unwrap() as *mut _ as *mut c_void
        }
        Values::DECIMAL(n) => {
            dp.push(n);
            pp.push(dp.last_mut().unwrap() as *mut _ as *mut c_void);
            pp.last_mut().unwrap() as *mut _ as *mut c_void
        }
        _ => panic!("unsupported type {:?} for extern pointers", val.typ),
    }
}

pub fn call_ffi(_extern: &Extern, stack: &mut Vec<Frame>, cur_frame: usize) {
    unsafe {
        let lib = Library::new(&_extern.dll).unwrap();

        let func: Symbol<*mut c_void> = lib.get(_extern.name.as_bytes()).unwrap();

        let code_ptr = CodePtr::from_ptr(func.clone().into_raw().as_raw_ptr());

        let args = stack[cur_frame].pop_args(_extern.arg_types.len());

        let mut types: Vec<ffi_type> = Vec::new();

        let mut arg_types: Vec<*mut ffi_type> = Vec::new();
        for typ in &_extern.arg_types {
            let arg_type = type_to_type(typ);
            types.push(arg_type);
            arg_types.push(types.last_mut().unwrap() as *mut ffi_type);
        }

        let mut cif: ffi_cif = Default::default();

        let mut ret_type = type_to_type(&_extern.ret_type);

        let mut raw_args: Vec<*mut c_void> = Vec::new();

        let mut signed_args: Vec<i64> = Vec::new();
        let mut unsigned_args: Vec<u64> = Vec::new();
        let mut decimal_args: Vec<f64> = Vec::new();

        let mut sp: Vec<i64> = Vec::new();
        let mut up: Vec<u64> = Vec::new();
        let mut dp: Vec<f64> = Vec::new();
        let mut pp: Vec<*mut c_void> = Vec::new();

        for arg in &args {
            match &arg.val {
                Values::SIGNED(n) => {
                    signed_args.push(*n);
                    raw_args.push(signed_args.last_mut().unwrap() as *mut _ as *mut c_void);
                }
                Values::UNSIGNED(n) => {
                    unsigned_args.push(*n);
                    raw_args.push(unsigned_args.last_mut().unwrap() as *mut _ as *mut c_void);
                }
                Values::DECIMAL(n) => {
                    decimal_args.push(*n);
                    raw_args.push(decimal_args.last_mut().unwrap() as *mut _ as *mut c_void);
                }
                Values::POINTER(p) => {
                    let val = stack[0].stack[*p].clone();
                    let ptr = get_pointer(&val, &mut sp, &mut up, &mut dp, &mut pp);
                    raw_args.push(ptr);
                }
                _ => panic!("unsupported type {:?} for externs", arg.typ),
            }
        }
        
        prep_cif(&mut cif, ffi_abi_FFI_DEFAULT_ABI, _extern.arg_types.len(), addr_of_mut!(ret_type), arg_types.as_mut_ptr()).unwrap();

        let val = match _extern.ret_type.typ[0] {
            Types::VOID => {
                call::<c_void>(&mut cif, code_ptr, raw_args.as_mut_ptr());
                Values::VOID
            },
            Types::I8 => {
                let result: i64 = call::<i8>(&mut cif, code_ptr, raw_args.as_mut_ptr()) as i64;
                Values::SIGNED(result)
            },
            Types::I16 => {
                let result: i64 = call::<i16>(&mut cif, code_ptr, raw_args.as_mut_ptr()) as i64;
                Values::SIGNED(result)
            },
            Types::I32 => {
                let result: i64 = call::<i32>(&mut cif, code_ptr, raw_args.as_mut_ptr()) as i64;
                Values::SIGNED(result)
            },
            Types::I64 => {
                let result: i64 = call::<i64>(&mut cif, code_ptr, raw_args.as_mut_ptr()) as i64;
                Values::SIGNED(result)
            },
            Types::U8 => {
                let result: u64 = call::<u8>(&mut cif, code_ptr, raw_args.as_mut_ptr()) as u64;
                Values::UNSIGNED(result)
            },
            Types::U16 => {
                let result: u64 = call::<u16>(&mut cif, code_ptr, raw_args.as_mut_ptr()) as u64;
                Values::UNSIGNED(result)
            },
            Types::U32 => {
                let result: u64 = call::<u32>(&mut cif, code_ptr, raw_args.as_mut_ptr()) as u64;
                Values::UNSIGNED(result)
            },
            Types::U64 => {
                let result: u64 = call::<u64>(&mut cif, code_ptr, raw_args.as_mut_ptr()) as u64;
                Values::UNSIGNED(result)
            },
            Types::F32 => {
                let result: f64 = call::<f32>(&mut cif, code_ptr, raw_args.as_mut_ptr()) as f64;
                Values::DECIMAL(result)
            },
            Types::F64 => {
                let result: f64 = call::<f64>(&mut cif, code_ptr, raw_args.as_mut_ptr());
                Values::DECIMAL(result)
            },
            Types::POINTER => {
                match _extern.ret_type.typ[1] {
                    Types::VOID => {
                        let result: *const c_void = call::<*const c_void>(&mut cif, code_ptr, raw_args.as_mut_ptr());
                        Values::UNSIGNED(result as u64)
                    }
                    _ => panic!("unsupported return type {:?}", _extern.ret_type),
                }
            }
            _ => panic!("unsupported return type {:?}", _extern.ret_type),
        };

        stack[cur_frame].push(Value { typ: _extern.ret_type.clone(), val });
    }
}
