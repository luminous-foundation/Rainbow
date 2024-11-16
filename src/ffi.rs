use libloading::{Library, Symbol};
use libffi::{low::*, raw::FFI_TYPE_STRUCT};
use std::{ffi::c_void, ptr::{addr_of_mut, null_mut}};
use crate::{_struct::Struct, _type::{Type, Types}, frame::Frame, function::Extern, get_struct, value::{Value, Values}, scope::Scope};

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
        _ => panic!("unsupported type `{:?}` for externs", typ.typ[0])
    }
}

pub unsafe fn struct_to_ffi(_struct: Struct, types: &mut Vec<ffi_type>, var_type_storage: &mut Vec<Vec<*mut ffi_type>>) -> ffi_type {
    let mut var_types: Vec<*mut ffi_type> = Vec::new();

    let size = get_struct_size(&_struct);

    for typ in _struct.var_types {
        let mut _typ = type_to_type(&typ);
        types.push(_typ);
        var_types.push(types.last_mut().unwrap() as *mut ffi_type);
    }

    var_types.push(null_mut());

    let typ = ffi_type {
        size,
        alignment: 0,
        type_: FFI_TYPE_STRUCT as u16,
        elements: var_types.as_mut_ptr(),
    };

    var_type_storage.push(var_types);

    return typ;
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
        _ => panic!("unsupported type `{:?}` for extern pointers", vals[0].typ.typ),
    }
}

pub fn call_ffi(_extern: &Extern, stack: &mut Vec<Frame>, cur_frame: usize, global_frame: usize, scope: &Scope, global_scope: &Scope) {
    unsafe {
        let lib = Library::new(&_extern.dll).unwrap();

        let func: Symbol<*mut c_void> = lib.get(_extern.name.as_bytes()).unwrap();

        let code_ptr = CodePtr::from_ptr(func.clone().into_raw().as_raw_ptr());
        
        let args = stack[cur_frame].pop_args(_extern.arg_types.len(), global_scope, scope);

        let mut types: Vec<ffi_type> = Vec::new();

        let mut var_type_storage: Vec<Vec<*mut ffi_type>> = Vec::new();

        let mut arg_types: Vec<*mut ffi_type> = Vec::new();
        let mut index = 0;
        for typ in &_extern.arg_types {
            match typ.typ[0] {
                Types::STRUCT => {
                    match &args[index][0].val {
                        Values::STRUCT(module, name, _) => {
                            let _struct = get_struct(&module, &name, global_scope, scope);
                            
                            let arg_type = struct_to_ffi(_struct, &mut types, &mut var_type_storage); 
                            types.push(arg_type);
                            arg_types.push(types.last_mut().unwrap() as *mut ffi_type);
                        }
                        _ => panic!("illegal type created, type is `STRUCT` value is {:?}", args[index])
                    }
                }
                _ => {
                    let arg_type = type_to_type(typ);
                    types.push(arg_type);
                    arg_types.push(types.last_mut().unwrap() as *mut ffi_type);
                }
            }
            index += 1;
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

        let mut struct_data: Vec<Vec<u8>> = Vec::new();

        for arg in &args {
            match &arg[0].val {
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
                    let val = &stack[global_frame].stack[*p..*p+*s];
                    let ptr = get_pointer(val, &mut pp, &mut s8p, &mut s16p, &mut s32p, &mut s64p, &mut u8p, &mut u16p, &mut u32p, &mut u64p, &mut f32p, &mut f64p);

                    raw_args.push(ptr);
                }
                Values::STRUCT(module, name, _) => {
                    let struct_type = get_struct(module, name, global_scope, scope);

                    let struct_size = get_struct_size(&struct_type);
                    let mut struct_bytes = vec![0u8; struct_size];

                    let mut offset = 0;
                    let mut i = 0;
                    for _ in struct_type.var_types {
                        let val = &arg[i + 1];
                        
                        let val_ptr = struct_bytes.as_mut_ptr().add(offset);

                        match val.val {
                            Values::SIGNED(num) => {
                                match val.typ.typ[0] {
                                    Types::I8  => *(val_ptr as *mut i8)  = num as i8,
                                    Types::I16 => *(val_ptr as *mut i16) = num as i16,
                                    Types::I32 => *(val_ptr as *mut i32) = num as i32,
                                    Types::I64 => *(val_ptr as *mut i64) = num as i64,
                                    _ => panic!("illegal type created, type is `SIGNED` value is {:?}", val)
                                }
                                offset += val.typ.typ[0].get_size();
                            }
                            Values::UNSIGNED(num) => {
                                match val.typ.typ[0] {
                                    Types::U8  => *(val_ptr as *mut u8)  = num as u8,
                                    Types::U16 => *(val_ptr as *mut u16) = num as u16,
                                    Types::U32 => *(val_ptr as *mut u32) = num as u32,
                                    Types::U64 => *(val_ptr as *mut u64) = num as u64,
                                    _ => panic!("illegal type created, type is `SIGNED` value is {:?}", val)
                                }
                                offset += val.typ.typ[0].get_size();
                            }
                            Values::DECIMAL(num) => {
                                match val.typ.typ[0] {
                                    Types::F16 => todo!("f16 not supported by ffi yet"), 
                                    Types::F32 => *(val_ptr as *mut f32) = num as f32,
                                    Types::F64 => *(val_ptr as *mut f64) = num as f64,
                                    _ => panic!("illegal type created, type is `SIGNED` value is {:?}", val)
                                }
                                offset += val.typ.typ[0].get_size();
                            }
                            _ => todo!("unsupported value in struct {:?}", val)
                        }

                        i += 1;
                    }

                    raw_args.push(struct_bytes.as_mut_ptr() as *mut c_void);
                    struct_data.push(struct_bytes);
                }
                _ => panic!("unsupported type `{:?}` for externs (value: `{:?}`)", arg[0].typ, arg[0].val),
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
                    _ => panic!("unsupported return type `{:?}`", _extern.ret_type),
                }
            }
            Types::STRUCT => {
                todo!("struct return types are not yet implemented");
            }
            _ => panic!("unsupported return type `{:?}`", _extern.ret_type),
        };

        for i in 0..arg_types.len() {
            let arg = raw_args[i];
            let typ = _extern.arg_types[i].clone();

            match typ.typ[0] {
                Types::POINTER => {
                    let index;
                    let len;
                    match args[i][0].val {
                        Values::POINTER(p, s) => {
                            index = p;
                            len = s;
                        }
                        _ => panic!("type mismatch, expected POINTER got {:?} (value: {:?})", args[i][0].typ, args[i][0].val)
                    }

                    let pointer = *(arg as *mut *mut c_void);
                    for j in 0..len {
                        let pointer_val = match typ.typ[1] {
                            Types::VOID => &Values::VOID,
                            Types::I8 => &Values::SIGNED(*(pointer.wrapping_add(j) as *mut i8) as i64),
                            Types::I16 => &Values::SIGNED(*(pointer.wrapping_add(j) as *mut i16) as i64),
                            Types::I32 => &Values::SIGNED(*(pointer.wrapping_add(j) as *mut i32) as i64),
                            Types::I64 => &Values::SIGNED(*(pointer.wrapping_add(j) as *mut i64) as i64),
                            Types::U8 => &Values::UNSIGNED(*(pointer.wrapping_add(j) as *mut u8) as u64),
                            Types::U16 => &Values::UNSIGNED(*(pointer.wrapping_add(j) as *mut u16) as u64),
                            Types::U32 => &Values::UNSIGNED(*(pointer.wrapping_add(j) as *mut u32) as u64),
                            Types::U64 => &Values::UNSIGNED(*(pointer.wrapping_add(j) as *mut u64) as u64),
                            Types::F16 => todo!(),
                            Types::F32 => &Values::DECIMAL(*(pointer.wrapping_add(j) as *mut f32) as f64),
                            Types::F64 => &Values::DECIMAL(*(pointer.wrapping_add(j) as *mut f64) as f64),
                            Types::POINTER => todo!(),
                            Types::TYPE => todo!(),
                            Types::STRUCT => todo!(),
                            Types::NAME => todo!(),
                        };

                        stack[global_frame].stack[index + j].set(pointer_val);
                    }
                }
                _ => ()
            }
        }

        stack[cur_frame].push(Value { typ: _extern.ret_type.clone(), val });
    }
}

fn get_struct_size(_struct: &Struct) -> usize {
    let mut size = 0;

    for typ in &_struct.var_types {
        size += typ.get_size();
    }

    return size;
}
