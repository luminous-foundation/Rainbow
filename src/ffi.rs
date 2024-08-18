use libloading::{Library, Symbol};
use libffi::low::*;
use std::{ffi::c_void, ptr::addr_of_mut};
use crate::{_type::{Type, Types}, frame::Frame, function::Extern, value::{Value, Values}};

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

macro_rules! push_ptr {
    ($typ:tt, $vals:expr, $v:expr, $pp:expr, $typ2:tt) => {
        let mut v: Vec<$typ> = Vec::new();
        for val in $vals {
            match val.val {
                Values::$typ2(n) => v.push(n as $typ),
                _ => panic!("pointers with multiple types are not supported")
            }
        }
        $v.push(v.clone());
        $pp.push($v.last_mut().unwrap().as_mut_ptr() as *mut _ as *mut c_void);
        return $pp.last_mut().unwrap() as *mut _ as *mut c_void
    }
}

// rust has forced my hand with this one
pub unsafe fn get_pointer(vals: &[Value], pp: &mut Vec<*mut c_void>, s8p: &mut Vec<Vec<i8>>, s16p: &mut Vec<Vec<i16>>, s32p: &mut Vec<Vec<i32>>, s64p: &mut Vec<Vec<i64>>, u8p: &mut Vec<Vec<u8>>, u16p: &mut Vec<Vec<u16>>, u32p: &mut Vec<Vec<u32>>, u64p: &mut Vec<Vec<u64>>, f32p: &mut Vec<Vec<f32>>, f64p: &mut Vec<Vec<f64>>) -> *mut c_void {
    match vals[0].typ.typ[0] {
        Types::I8  => { push_ptr!(i8, vals,  s8p,  pp, SIGNED);   }
        Types::I16 => { push_ptr!(i16, vals, s16p, pp, SIGNED);   }
        Types::I32 => { push_ptr!(i32, vals, s32p, pp, SIGNED);   }
        Types::I64 => { push_ptr!(i64, vals, s64p, pp, SIGNED);   }
        Types::U8  => { push_ptr!(u8, vals,  u8p,  pp, UNSIGNED); }
        Types::U16 => { push_ptr!(u16, vals, u16p, pp, UNSIGNED); }
        Types::U32 => { push_ptr!(u32, vals, u32p, pp, UNSIGNED); }
        Types::U64 => { push_ptr!(u64, vals, u64p, pp, UNSIGNED); }
        Types::F32 => { push_ptr!(f32, vals, f32p, pp, DECIMAL);  }
        Types::F64 => { push_ptr!(f64, vals, f64p, pp, DECIMAL);  }
        _ => panic!("unsupported type {:?} for extern pointers", vals[0].typ.typ),
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

        
        let mut pp: Vec<*mut c_void> = Vec::new();
        let mut s8p: Vec<Vec<i8>> = Vec::new();
        let mut s16p: Vec<Vec<i16>> = Vec::new();
        let mut s32p: Vec<Vec<i32>> = Vec::new();
        let mut s64p: Vec<Vec<i64>> = Vec::new();
        let mut u8p: Vec<Vec<u8>> = Vec::new();
        let mut u16p: Vec<Vec<u16>> = Vec::new();
        let mut u32p: Vec<Vec<u32>> = Vec::new();
        let mut u64p: Vec<Vec<u64>> = Vec::new();
        let mut f32p: Vec<Vec<f32>> = Vec::new();
        let mut f64p: Vec<Vec<f64>> = Vec::new();

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
                Values::POINTER(p, s) => {
                    let val = &stack[0].stack[*p..*p+*s];
                    let ptr = get_pointer(val, &mut pp, &mut s8p, &mut s16p, &mut s32p, &mut s64p, &mut u8p, &mut u16p, &mut u32p, &mut u64p, &mut f32p, &mut f64p);

                    // dbg!(*(*(ptr as *mut *mut u8) as *mut u8));
                    // dbg!(*(ptr as *mut *mut u8) as *mut u8);
                    // dbg!(ptr as *mut *mut u8);

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

        // if args.len() > 2 {
        //     let ptr = raw_args[1];
        //     dbg!(*(*(ptr as *mut *mut u8) as *mut u8));
        //     dbg!(*(ptr as *mut *mut u8) as *mut u8);
        //     dbg!(ptr as *mut *mut u8);
        // }
    }
}
