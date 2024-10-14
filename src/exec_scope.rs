use std::collections::HashMap;

use crate::{_type::Types, block::Block, ffi::call_ffi, frame::Frame, func_exists, function::Function, get_extern, get_func, get_struct, get_var, instruction::{Instruction, Opcode}, scope::Scope, set_var, value::Values};

// instruction macros
macro_rules! peek {
    ($val:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        let index;
        match $val.val {
            Values::SIGNED(n) => index = n as usize,
            Values::UNSIGNED(n) => index = n as usize,
            Values::DECIMAL(n) => index = n as usize,
            Values::POINTER(n, _) => index = n as usize,
            _ => panic!("cannot peek using a non-numeral value index"),
        }

        let val = $stack[$cur_frame].stack[index].val.clone();
        set_var($out, &val, $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
    }
}

macro_rules! call {
    ($func:expr, $scope:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        if func_exists($func, $scope, $global_scope) {
            let frame_func = get_func($func, $scope, $global_scope, $module_frame, $global_frame);
            let frame = frame_func.0;
            let func = frame_func.1;
            exec_func(&func, $global_scope, $stack, frame, $global_frame);
        } else {
            let func = get_extern($func, $scope, $global_scope);
            call_ffi(func, $stack, $cur_frame, $global_frame);
        }
    }
}

macro_rules! add {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        let val = $a.val.add(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
    };
}
macro_rules! sub {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        let val = $a.val.sub(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
    };
}
macro_rules! mul {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        let val = $a.val.mul(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
    };
}
macro_rules! div {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        let val = $a.val.div(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
    };
}
macro_rules! modulo {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        let val = $a.val.modulo(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
    };
}

macro_rules! compare {
    ($a_val:expr, $b:expr, $op:tt, $pc:expr, $new_pc:expr, $skip_inc:expr) => {
        match $b {
            Values::SIGNED(b_val) => {
                if ($a_val as i64) $op b_val {
                    $skip_inc = true;
                    $pc = $new_pc as usize;
                }
            }
            Values::UNSIGNED(b_val) => {
                if ($a_val as u64) $op b_val {
                    $skip_inc = true;
                    $pc = $new_pc as usize;
                }
            }
            Values::DECIMAL(b_val) => {
                if ($a_val as f64) $op b_val {
                    $skip_inc = true;
                    $pc = $new_pc as usize;
                }
            }
            _ => panic!("expected a number for comparison, got `{:?}`", $b),
        }
    }
}
macro_rules! get_pc {
    ($c:expr, $new_pc:expr) => {
        match $c {
            Values::UNSIGNED(c_val) => $new_pc = c_val as usize,
            Values::SIGNED(c_val) => {
                if c_val < 0 {
                    panic!("cannot jump to negative address");
                } else {
                    $new_pc = c_val as usize;
                }
            }
            _ => panic!("expected integer address value")
        }
    }
}

macro_rules! jne {
    ($a:expr, $b:expr, $c:expr, $pc:expr, $skip_inc:expr) => {
        let new_pc;
        get_pc!($c.val, new_pc);

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, !=, $pc, new_pc, $skip_inc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, !=, $pc, new_pc, $skip_inc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, !=, $pc, new_pc, $skip_inc),
            _ => panic!("expected a number for comparison, got `{:?}`", $a.val)
        }
    }
}
macro_rules! je {
    ($a:expr, $b:expr, $c:expr, $pc:expr, $skip_inc:expr) => {
        let new_pc;
        get_pc!($c.val, new_pc);

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, ==, $pc, new_pc, $skip_inc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, ==, $pc, new_pc, $skip_inc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, ==, $pc, new_pc, $skip_inc),
            _ => panic!("expected a number for comparison, got `{:?}`", $a.val)
        }
    }
}

macro_rules! jge {
    ($a:expr, $b:expr, $c:expr, $pc:expr, $skip_inc:expr) => {
        let new_pc;
        get_pc!($c.val, new_pc);

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, >=, $pc, new_pc, $skip_inc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, >=, $pc, new_pc, $skip_inc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, >=, $pc, new_pc, $skip_inc),
            _ => panic!("expected a number for comparison, got `{:?}`", $a.val)
        }
    }
}
macro_rules! jg {
    ($a:expr, $b:expr, $c:expr, $pc:expr, $skip_inc:expr) => {
        let new_pc;
        get_pc!($c.val, new_pc);

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, >, $pc, new_pc, $skip_inc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, >, $pc, new_pc, $skip_inc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, >, $pc, new_pc, $skip_inc),
            _ => panic!("expected a number for comparison, got `{:?}`", $a.val)
        }
    }
}
macro_rules! jle {
    ($a:expr, $b:expr, $c:expr, $pc:expr, $skip_inc:expr) => {
        let new_pc;
        get_pc!($c.val, new_pc);

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, <=, $pc, new_pc, $skip_inc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, <=, $pc, new_pc, $skip_inc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, <=, $pc, new_pc, $skip_inc),
            _ => panic!("expected a number for comparison, got `{:?}`", $a.val)
        }
    }
}
macro_rules! jl {
    ($a:expr, $b:expr, $c:expr, $pc:expr, $skip_inc:expr) => {
        let new_pc;
        get_pc!($c.val, new_pc);

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, <, $pc, new_pc, $skip_inc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, <, $pc, new_pc, $skip_inc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, <, $pc, new_pc, $skip_inc),
            _ => panic!("expected a number for comparison, got `{:?}`", $a.val)
        }
    }
}

macro_rules! mov {
    ($a:expr, $b:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        set_var($b, &$a.val, $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
    }
}

macro_rules! and {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        let val = $a.val.and(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
    };
}

macro_rules! or {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        let val = $a.val.or(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
    };
}

macro_rules! xor {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        let val = $a.val.xor(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
    };
}

macro_rules! not {
    ($a:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        let val = $a.val.not();
        set_var($out, &val, $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
    };
}

macro_rules! lsh {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        let val = $a.val.lsh(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
    };
}

macro_rules! rsh {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        let val = $a.val.rsh(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
    };
}

macro_rules! get_type {
    ($typ:expr, $type_var:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $action:expr, $module_frame:expr, $global_frame:expr) => {
        let type_var = get_var($type_var, $global_scope, $stack, $cur_frame, $module_frame, $global_frame);

        match &type_var.val {
            Values::TYPE(t) => $typ = t.clone(),
            _ => panic!("tried to {} with dynamic type stored in variable, but given variable had type `{:?}`", $action, type_var.typ)
        }
    }
}
macro_rules! get_name {
    ($name:expr, $name_var:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $action:expr, $module_frame:expr, $global_frame:expr) => {
        let name_var = get_var($name_var, $global_scope, $stack, $cur_frame, $module_frame, $global_frame);

        match &name_var.val {
            Values::NAME(n) => $name = n.clone(),
            _ => panic!("tried to {} variable with dynamic name stored in variable, but given variable had type `{:?}`", $action, name_var.typ)
        }
    }
}

macro_rules! ref_ {
    ($index:expr, $out_var:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        // ugly line
        let out_var_type = get_var($out_var, $global_scope, $stack, $cur_frame, $module_frame, $global_frame).typ.typ[0].clone();
        match out_var_type {
            Types::POINTER => {
                set_var($out_var, &Values::POINTER($index, 1), $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
            }
            _ => panic!("attempted set a variable with type `{:?}` to a reference", out_var_type)
        }
    }
}

macro_rules! deref {
    ($ptr:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        let index;
        match $ptr.val {
            Values::SIGNED(p) => index = p as usize,
            Values::UNSIGNED(p) => index = p as usize,
            Values::DECIMAL(p) => index = p as usize,
            Values::POINTER(p, _) => index = p,
            _ => panic!("attempted to deref non-pointer value")
        }
        
        let val = $stack[$global_frame].stack[index].val.clone();
        set_var($out, &val, $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
    }
}

macro_rules! get_usize {
    ($index:expr, $amnt:expr, $action:expr, $type:expr) => {
        $index = match($amnt.val) {
            Values::SIGNED(n) => n as usize,
            Values::UNSIGNED(n) => n as usize,
            Values::DECIMAL(n) => n as usize,
            Values::POINTER(n, _) => n,
            _ => panic!("cannot `{}` with non-number value as {}", $action, $type),
        };
    }
}

macro_rules! pmov {
    ($val:expr, $ptr:expr, $offset:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        let ptr = get_var($ptr, $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
        let ptr = match(ptr.val) {
            Values::SIGNED(p) => p as usize,
            Values::UNSIGNED(p) => p as usize,
            Values::DECIMAL(p) => p as usize,
            Values::POINTER(p, _) => p,
            _ => panic!("cannot PMOV into a non-pointer variable")
        };

        let offset_index;
        get_usize!(offset_index, $offset, "PMOV", "offset");

        if ptr + offset_index > $stack[$global_frame].stack.len() {
            panic!("`PMOV` index out of bounds: {} > {}", ptr + offset_index, $stack[$global_frame].stack.len() - 1);
        }

        $stack[$global_frame].stack[ptr + offset_index].set(&$val.val);
    }
}

macro_rules! alloc {
    ($typ:expr, $amnt:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        let amnt = match($amnt.val) {
            Values::SIGNED(n) => n as u64,
            Values::UNSIGNED(n) => n,
            Values::DECIMAL(n) => n as u64,
            _ => panic!("cannot allocate with non-number value as count"),
        };

        let index = $stack[$global_frame].stack.len();

        $stack[$cur_frame].set_var($out, &Values::POINTER(index, amnt as usize));

        for _ in 0..amnt {
            $stack[$global_frame].push_alloc($typ, $out.clone());
        }
    }
}

macro_rules! free_ {
    ($ptr:expr, $amnt:expr, $stack:expr, $global_frame:expr) => {
        let mut index;
        get_usize!(index, $ptr, "free", "pointer");

        let size;
        get_usize!(size, $amnt, "free", "size");

        let start = index;

        // TODO: this loop will get extremely slow with large allocs
        //       replace this with full heap reconstruction, or somehow allow the heap to get fragmented
        for _ in 0..size {
            $stack[$global_frame].allocs.remove(start);
            $stack[$global_frame].stack.remove(start);
            index += 1;
        }
    }
}

macro_rules! cmp {
    ($cond:expr, $a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $module_frame:expr, $global_frame:expr) => {
        {
            let c;
            match $cond.val {
                Values::SIGNED(n) => c = n as u64,
                Values::UNSIGNED(n) => c = n,
                Values::DECIMAL(n) => c = n as u64,
                _ => panic!("invalid condition `{:?}` passed to `CMP` instruction", $cond.val)
            }

            match c {
                0x00 => {
                    if $a.val == $b.val {
                        set_var($out, &Values::UNSIGNED(1), $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
                    } else {
                        set_var($out, &Values::UNSIGNED(0), $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
                    }
                }
                0x01 => {
                    if $a.val != $b.val {
                        set_var($out, &Values::UNSIGNED(1), $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
                    } else {
                        set_var($out, &Values::UNSIGNED(0), $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
                    }
                }
                0x02 => {
                    if $a.val >= $b.val {
                        set_var($out, &Values::UNSIGNED(1), $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
                    } else {
                        set_var($out, &Values::UNSIGNED(0), $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
                    }
                }
                0x03 => {
                    if $a.val > $b.val {
                        set_var($out, &Values::UNSIGNED(1), $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
                    } else {
                        set_var($out, &Values::UNSIGNED(0), $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
                    }
                }
                0x04 => {
                    if $a.val <= $b.val {
                        set_var($out, &Values::UNSIGNED(1), $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
                    } else {
                        set_var($out, &Values::UNSIGNED(0), $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
                    }
                }
                0x05 => {
                    if $a.val < $b.val {
                        set_var($out, &Values::UNSIGNED(1), $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
                    } else {
                        set_var($out, &Values::UNSIGNED(0), $global_scope, $stack, $cur_frame, $module_frame, $global_frame);
                    }
                }
                _ => panic!("invalid condition `{:#04x}` passed to `CMP` instruction", c)
            }
        }
    };
}

pub fn exec_block(scope: &Scope, block: &Vec<Instruction>, global_scope: &Scope, stack: &mut Vec<Frame>, cur_frame: usize, pc: &mut usize, block_start: usize, module_frame: usize, global_frame: usize) -> i32 {
    // i want to make per-instruction timing toggleable
    // but i also want to do it in a way that doesnt have any performance impact
    // i'll have to figure out a way
    // let mut times: [f64; 256] = [0f64; 256];
    // let mut counts: [u32; 256] = [0; 256];

    // let start = std::time::Instant::now();

    let mut skip_inc = false;

    while *pc - block_start < block.len() {
        let instr = &block[*pc - block_start];

        // let instr_start = std::time::Instant::now();
        match &instr.opcode {
            Opcode::NOP => { // NOP
                // do nothing
            }

            Opcode::PUSH_IMM(val) => { // PUSH [imm]
                stack[cur_frame].push(val.clone());
            }
            Opcode::PUSH_VAR(name) => { // PUSH [var]
                let var = get_var(name, global_scope, stack, cur_frame, module_frame, global_frame);

                let val = var.clone();
                stack[cur_frame].push(val);
            }

            Opcode::POP(name) => { // POP [var]
                set_var(name, &stack[cur_frame].pop().val, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::PEEK_IMM(val, out) => { // PEEK [imm] [var]
                peek!(val, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::PEEK_VAR(val_var, out) => { // PEEK [var] [var]
                let val = get_var(val_var, global_scope, stack, cur_frame, module_frame, global_frame);

                peek!(val, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::CALL_FUNC(func) => { // CALL [func]
                call!(func, scope, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::CALL_VAR(func_var) => { // CALL [var]
                let func_var = get_var(func_var, global_scope, stack, cur_frame, module_frame, global_frame);

                let func;
                match &func_var.val {
                    Values::NAME(n) => func = n,
                    _ => panic!("tried to call function with name stored in variable, but given variable had type `{:?}`", func_var.typ)
                }

                call!(func, scope, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::ADD_I_I(a, b, out) => { // ADD [imm] [imm] [var]
                add!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::ADD_V_I(a_name, b, out) => { // ADD [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                add!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::ADD_I_V(a, b_name, out) => { // ADD [imm] [var] [var]                
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                add!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::ADD_V_V(a_name, b_name, out) => { // ADD [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                add!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::SUB_I_I(a, b, out) => { // SUB [imm] [imm] [var]
                sub!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::SUB_V_I(a_name, b, out) => { // SUB [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                sub!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::SUB_I_V(a, b_name, out) => { // SUB [imm] [var] [var]                
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                sub!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::SUB_V_V(a_name, b_name, out) => { // SUB [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                sub!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::MUL_I_I(a, b, out) => { // MUL [imm] [imm] [var]
                mul!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::MUL_V_I(a_name, b, out) => { // MUL [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                mul!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::MUL_I_V(a, b_name, out) => { // MUL [imm] [var] [var]                
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                mul!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::MUL_V_V(a_name, b_name, out) => { // MUL [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                mul!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::DIV_I_I(a, b, out) => { // DIV [imm] [imm] [var]
                div!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::DIV_V_I(a_name, b, out) => { // DIV [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                div!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::DIV_I_V(a, b_name, out) => { // DIV [imm] [var] [var]                
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                div!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::DIV_V_V(a_name, b_name, out) => { // DIV [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                div!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::JMP_IMM(new_pc_val) => { // JMP [imm]
                let new_pc: usize;
                get_pc!(new_pc_val.val.clone(), new_pc);

                *pc = new_pc;
                skip_inc = true;
            }
            Opcode::JMP_VAR(new_pc_name) => { // JMP [var]
                let new_pc_var = get_var(new_pc_name, global_scope, stack, cur_frame, module_frame, global_frame).val.clone();
                let new_pc: usize;
                get_pc!(new_pc_var, new_pc);

                *pc = new_pc;
                skip_inc = true;
            }

            Opcode::JNE_I_I_I(a, b, c) => { // JNE [imm] [imm] [imm]
                jne!(a, b, c, *pc, skip_inc);
            }
            Opcode::JNE_V_I_I(a_name, b, c) => { // JNE [var] [imm] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jne!(a, b, c, *pc, skip_inc);
            }
            Opcode::JNE_I_V_I(a, b_name, c) => { // JNE [imm] [imm] [imm]
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jne!(a, b, c, *pc, skip_inc);
            }
            Opcode::JNE_V_V_I(a_name, b_name, c) => { // JNE [var] [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jne!(a, b, c, *pc, skip_inc);
            }
            Opcode::JNE_I_I_V(a, b, c_name) => { // JNE [imm] [imm] [var]
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jne!(a, b, c, *pc, skip_inc);
            }
            Opcode::JNE_V_I_V(a_name, b, c_name) => { // JNE [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jne!(a, b, c, *pc, skip_inc);
            }
            Opcode::JNE_I_V_V(a, b_name, c_name) => { // JNE [imm] [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jne!(a, b, c, *pc, skip_inc);
            }
            Opcode::JNE_V_V_V(a_name, b_name, c_name) => { // JNE [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jne!(a, b, c, *pc, skip_inc);
            }

            Opcode::JE_I_I_I(a, b, c) => { // JE [imm] [imm] [imm]
                je!(a, b, c, *pc, skip_inc);
            }
            Opcode::JE_V_I_I(a_name, b, c) => { // JE [var] [imm] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                je!(a, b, c, *pc, skip_inc);
            }
            Opcode::JE_I_V_I(a, b_name, c) => { // JE [imm] [imm] [imm]
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                je!(a, b, c, *pc, skip_inc);
            }
            Opcode::JE_V_V_I(a_name, b_name, c) => { // JE [var] [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                je!(a, b, c, *pc, skip_inc);
            }
            Opcode::JE_I_I_V(a, b, c_name) => { // JE [imm] [imm] [var]
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                je!(a, b, c, *pc, skip_inc);
            }
            Opcode::JE_V_I_V(a_name, b, c_name) => { // JE [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                je!(a, b, c, *pc, skip_inc);
            }
            Opcode::JE_I_V_V(a, b_name, c_name) => { // JE [imm] [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                je!(a, b, c, *pc, skip_inc);
            }
            Opcode::JE_V_V_V(a_name, b_name, c_name) => { // JE [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                je!(a, b, c, *pc, skip_inc);
            }

            Opcode::JGE_I_I_I(a, b, c) => { // JGE [imm] [imm] [imm]
                jge!(a, b, c, *pc, skip_inc);
            }
            Opcode::JGE_V_I_I(a_name, b, c) => { // JGE [var] [imm] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jge!(a, b, c, *pc, skip_inc);
            }
            Opcode::JGE_I_V_I(a, b_name, c) => { // JGE [imm] [imm] [imm]
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jge!(a, b, c, *pc, skip_inc);
            }
            Opcode::JGE_V_V_I(a_name, b_name, c) => { // JGE [var] [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jge!(a, b, c, *pc, skip_inc);
            }
            Opcode::JGE_I_I_V(a, b, c_name) => { // JGE [imm] [imm] [var]
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jge!(a, b, c, *pc, skip_inc);
            }
            Opcode::JGE_V_I_V(a_name, b, c_name) => { // JGE [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jge!(a, b, c, *pc, skip_inc);
            }
            Opcode::JGE_I_V_V(a, b_name, c_name) => { // JGE [imm] [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jge!(a, b, c, *pc, skip_inc);
            }
            Opcode::JGE_V_V_V(a_name, b_name, c_name) => { // JGE [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jge!(a, b, c, *pc, skip_inc);
            }

            Opcode::JG_I_I_I(a, b, c) => { // JG [imm] [imm] [imm]
                jg!(a, b, c, *pc, skip_inc);
            }
            Opcode::JG_V_I_I(a_name, b, c) => { // JG [var] [imm] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jg!(a, b, c, *pc, skip_inc);
            }
            Opcode::JG_I_V_I(a, b_name, c) => { // JG [imm] [imm] [imm]
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jg!(a, b, c, *pc, skip_inc);
            }
            Opcode::JG_V_V_I(a_name, b_name, c) => { // JG [var] [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jg!(a, b, c, *pc, skip_inc);
            }
            Opcode::JG_I_I_V(a, b, c_name) => { // JG [imm] [imm] [var]
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jg!(a, b, c, *pc, skip_inc);
            }
            Opcode::JG_V_I_V(a_name, b, c_name) => { // JG [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jg!(a, b, c, *pc, skip_inc);
            }
            Opcode::JG_I_V_V(a, b_name, c_name) => { // JG [imm] [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jg!(a, b, c, *pc, skip_inc);
            }
            Opcode::JG_V_V_V(a_name, b_name, c_name) => { // JG [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jg!(a, b, c, *pc, skip_inc);
            }

            Opcode::JLE_I_I_I(a, b, c) => { // JLE [imm] [imm] [imm]
                jle!(a, b, c, *pc, skip_inc);
            }
            Opcode::JLE_V_I_I(a_name, b, c) => { // JLE [var] [imm] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jle!(a, b, c, *pc, skip_inc);
            }
            Opcode::JLE_I_V_I(a, b_name, c) => { // JLE [imm] [imm] [imm]
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jle!(a, b, c, *pc, skip_inc);
            }
            Opcode::JLE_V_V_I(a_name, b_name, c) => { // JLE [var] [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jle!(a, b, c, *pc, skip_inc);
            }
            Opcode::JLE_I_I_V(a, b, c_name) => { // JLE [imm] [imm] [var]
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jle!(a, b, c, *pc, skip_inc);
            }
            Opcode::JLE_V_I_V(a_name, b, c_name) => { // JLE [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jle!(a, b, c, *pc, skip_inc);
            }
            Opcode::JLE_I_V_V(a, b_name, c_name) => { // JLE [imm] [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jle!(a, b, c, *pc, skip_inc);
            }
            Opcode::JLE_V_V_V(a_name, b_name, c_name) => { // JLE [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jle!(a, b, c, *pc, skip_inc);
            }

            Opcode::JL_I_I_I(a, b, c) => { // JL [imm] [imm] [imm]
                jl!(a, b, c, *pc, skip_inc);
            }
            Opcode::JL_V_I_I(a_name, b, c) => { // JL [var] [imm] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jl!(a, b, c, *pc, skip_inc);
            }
            Opcode::JL_I_V_I(a, b_name, c) => { // JL [imm] [imm] [imm]
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jl!(a, b, c, *pc, skip_inc);
            }
            Opcode::JL_V_V_I(a_name, b_name, c) => { // JL [var] [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jl!(a, b, c, *pc, skip_inc);
            }
            Opcode::JL_I_I_V(a, b, c_name) => { // JL [imm] [imm] [var]
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jl!(a, b, c, *pc, skip_inc);
            }
            Opcode::JL_V_I_V(a_name, b, c_name) => { // JL [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jl!(a, b, c, *pc, skip_inc);
            }
            Opcode::JL_I_V_V(a, b_name, c_name) => { // JL [imm] [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jl!(a, b, c, *pc, skip_inc);
            }
            Opcode::JL_V_V_V(a_name, b_name, c_name) => { // JL [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                jl!(a, b, c, *pc, skip_inc);
            }

            Opcode::MOV_I_V(a, b) => { // MOV [imm] [var]
                mov!(a, b, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::MOV_V_V(a_name, b) => { // MOV [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                mov!(a, b, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::MOV_VV_V(a_var, b) => { // MOV [var var] [var]
                let a_name;
                get_name!(a_name, a_var, global_scope, stack, cur_frame, "access", module_frame, global_frame);

                let a = get_var(&a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                mov!(a, b, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::MOV_I_VV(a, b_var) => { // MOV [imm] [var var]
                let b;
                get_name!(b, b_var, global_scope, stack, cur_frame, "set", module_frame, global_frame);

                mov!(a, &b, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::MOV_V_VV(a_name, b_var) => { // MOV [var] [var var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                let b;
                get_name!(b, b_var, global_scope, stack, cur_frame, "set", module_frame, global_frame);

                mov!(a, &b, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::MOV_VV_VV(a_var, b_var) => { // MOV [var var] [var var]
                let a_name;
                get_name!(a_name, a_var, global_scope, stack, cur_frame, "access", module_frame, global_frame);

                let a = get_var(&a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                let b;
                get_name!(b, b_var, global_scope, stack, cur_frame, "set", module_frame, global_frame);

                mov!(a, &b, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::AND_I_I(a, b, out) => { // AND [imm] [imm]
                and!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::AND_V_I(a_name, b, out) => { // AND [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                
                and!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::AND_I_V(a, b_name, out) => { // AND [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                and!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::AND_V_V(a_name, b_name, out) => { // AND [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                and!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::OR_I_I(a, b, out) => { // OR [imm] [imm]
                or!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::OR_V_I(a_name, b, out) => { // OR [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                
                or!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::OR_I_V(a, b_name, out) => { // OR [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                or!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::OR_V_V(a_name, b_name, out) => { // OR [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                or!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::XOR_I_I(a, b, out) => { // XOR [imm] [imm]
                xor!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::XOR_V_I(a_name, b, out) => { // XOR [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                
                xor!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::XOR_I_V(a, b_name, out) => { // XOR [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                xor!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::XOR_V_V(a_name, b_name, out) => { // XOR [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                xor!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::NOT_IMM(a, out) => { // NOT [imm]
                not!(a, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::NOT_VAR(a_name, out) => { // NOT [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                
                not!(a, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::LSH_I_I(a, b, out) => { // LSH [imm] [imm]
                lsh!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::LSH_V_I(a_name, b, out) => { // LSH [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                
                lsh!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::LSH_I_V(a, b_name, out) => { // LSH [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                lsh!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::LSH_V_V(a_name, b_name, out) => { // LSH [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                lsh!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::RSH_I_I(a, b, out) => { // RSH [imm] [imm]
                rsh!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::RSH_V_I(a_name, b, out) => { // RSH [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                
                rsh!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::RSH_I_V(a, b_name, out) => { // RSH [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                rsh!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::RSH_V_V(a_name, b_name, out) => { // RSH [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                rsh!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            
            Opcode::VAR_TYPE_NAME(typ, name) => { // VAR [type] [name]
                stack[cur_frame].create_var(name.clone(), typ.clone());
            }
            Opcode::VAR_VAR_NAME(type_var, name) => { // VAR [var] [name]
                let typ;
                get_type!(typ, type_var, global_scope, stack, cur_frame, "create variable", module_frame, global_frame);
                
                stack[cur_frame].create_var(name.clone(), typ);
            }
            Opcode::VAR_TYPE_VAR(typ, name_var) => { // VAR [type] [var]
                let name;
                get_name!(name, name_var, global_scope, stack, cur_frame, "create", module_frame, global_frame);

                stack[cur_frame].create_var(name, typ.clone())
            }
            Opcode::VAR_VAR_VAR(type_var, name_var) => { // VAR [var] [var]
                let typ;
                get_type!(typ, type_var, global_scope, stack, cur_frame, "create variable", module_frame, global_frame);

                let name;
                get_name!(name, name_var, global_scope, stack, cur_frame, "create", module_frame, global_frame);

                stack[cur_frame].create_var(name, typ);
            }

            // TODO: return type checking
            Opcode::RET => { // RET
                break;
            }
            Opcode::RET_IMM(v) => { // RET [imm]
                if cur_frame != global_frame {
                    stack[cur_frame - 1].push(v.clone());
                } else {
                    match v.val {
                        Values::VOID => return 0,
                        Values::SIGNED(n) => return n as i32,
                        Values::UNSIGNED(n) => return n as i32,
                        Values::DECIMAL(n) => return n as i32,
                        Values::POINTER(n, _) => return n as i32,
                        Values::STRUCT(_, _) => return 0,
                        Values::TYPE(_) => return 0,
                        Values::NAME(_) => return 0,
                    }
                }
                break;
            }
            Opcode::RET_VAR(var) => { // RET [var]
                let v = get_var(var, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                
                if cur_frame != global_frame {
                    stack[cur_frame - 1].push(v);
                } else {
                    match v.val {
                        Values::VOID => return 0,
                        Values::SIGNED(n) => return n as i32,
                        Values::UNSIGNED(n) => return n as i32,
                        Values::DECIMAL(n) => return n as i32,
                        Values::POINTER(n, _) => return n as i32,
                        Values::STRUCT(_, _) => return 0,
                        Values::TYPE(_) => return 0,
                        Values::NAME(_) => return 0,
                    }
                }
                break;
            }

            Opcode::REF_IMM(val, out_var) => {
                let index = stack[global_frame].stack.len();

                stack[global_frame].push(val.clone());

                ref_!(index, out_var, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::REF_VAR(var, out_var) => {
                let index = stack[global_frame].stack.len();

                // we only need to move the variable to the heap if it isnt already on the heap
                // TODO: figure out a way to change the name of the created variable on the heap
                //       to remove the possibility of name collisions
                //       
                //       if you have a variable with a certain name and you REF it, if there
                //       is a global variable with the same name it wil overwrite it
                if !stack[global_frame].vars.contains_key(var) {
                    if stack[cur_frame].vars.contains_key(var) {
                        let orig_var = stack[cur_frame].get_var(var).clone();

                        stack[global_frame].push_var(var, orig_var.typ, orig_var.val);
                    } else {
                        panic!("attempted to create a reference to a variable that doesnt exist");
                    }
                }
                
                ref_!(index, out_var, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::DEREF_IMM(ptr, out) => {
                deref!(ptr, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::DEREF_VAR(ptr_var, out) => {
                let ptr = get_var(ptr_var, global_scope, stack, cur_frame, module_frame, global_frame);

                deref!(ptr, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::INST_NAME(struct_name, out) => { // INST [name] [var]
                let frame = &stack[cur_frame];
                let start_index = frame.stack.len();

                let struct_type = get_struct(struct_name, global_scope);

                let strct = Values::STRUCT(struct_name.clone(), start_index);

                set_var(out, &strct, global_scope, stack, cur_frame, module_frame, global_frame);

                for i in 0..struct_type.var_types.len() {
                    stack[cur_frame].push_type(&struct_type.var_types[i]);
                }
            }

            Opcode::MOD_I_I(a, b, out) => { // MOD [imm] [imm] [var]
                modulo!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::MOD_V_I(a_name, b, out) => { // MOD [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                modulo!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::MOD_I_V(a, b_name, out) => { // MOD [imm] [var] [var]                
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                modulo!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::MOD_V_V(a_name, b_name, out) => { // MOD [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                modulo!(a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::PMOV_IMM_IMM(val, ptr, offset) => {
                pmov!(val, ptr, offset, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::PMOV_VAR_IMM(val_var, ptr, offset) => {
                let val = get_var(val_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                pmov!(val, ptr, offset, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::PMOV_IMM_VAR(val, ptr, offset_var) => {
                let offset = get_var(offset_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                pmov!(val, ptr, offset, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::PMOV_VAR_VAR(val_var, ptr, offset_var) => {
                let offset = get_var(offset_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let val = get_var(val_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                
                pmov!(val, ptr, offset, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::ALLOC_TYPE_IMM(typ, amnt, out) => {
                alloc!(typ, amnt, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::ALLOC_VAR_IMM(type_var, amnt, out) => {
                let typ;
                get_type!(typ, type_var, global_scope, stack, cur_frame, "allocate", module_frame, global_frame);

                alloc!(&typ, amnt, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::ALLOC_TYPE_VAR(typ, amnt_var, out) => {
                let amnt = get_var(amnt_var, global_scope, stack, cur_frame, module_frame, global_frame);

                alloc!(typ, amnt, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::ALLOC_VAR_VAR(type_var, amnt_var, out) => {
                let typ;
                get_type!(typ, type_var, global_scope, stack, cur_frame, "allocate", module_frame, global_frame);

                let amnt = get_var(amnt_var, global_scope, stack, cur_frame, module_frame, global_frame);

                alloc!(&typ, amnt, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            Opcode::FREE_VAR(ptr) => {
                let mut index = *stack[global_frame].vars.get(ptr).unwrap_or_else(|| panic!("attempted to free non-existent pointer `{}`", ptr));
                let start = index;

                stack[global_frame].vars.remove(ptr);

                // TODO: this loop will get extremely slow with large allocs
                //       replace this with full heap reconstruction, or somehow allow the heap to get fragmented
                while &stack[global_frame].allocs[index] == ptr {
                    stack[global_frame].allocs.remove(start);
                    stack[global_frame].stack.remove(start);
                    index += 1;
                }
            }
            Opcode::FREE_IMM_IMM(ptr, amnt) => {
                free_!(ptr, amnt, stack, global_frame);
            }
            Opcode::FREE_VAR_IMM(ptr_var, amnt) => {
                let ptr = get_var(ptr_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                free_!(ptr, amnt, stack, global_frame);
            }
            Opcode::FREE_IMM_VAR(ptr, amnt_var) => {
                let amnt = get_var(amnt_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                free_!(ptr, amnt, stack, global_frame);
            }
            Opcode::FREE_VAR_VAR(ptr_var, amnt_var) => {
                let ptr = get_var(ptr_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let amnt = get_var(amnt_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                free_!(ptr, amnt, stack, global_frame);
            }

            Opcode::CMP_I_I_I(cond, a, b, out) => {
                cmp!(cond, a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::CMP_V_I_I(cond_var, a, b, out) => {
                let cond = get_var(cond_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                cmp!(cond, a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::CMP_I_V_I(cond, a_var, b, out) => {
                let a = get_var(a_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                cmp!(cond, a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::CMP_V_V_I(cond_var, a_var, b, out) => {
                let cond = get_var(cond_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let a = get_var(a_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                cmp!(cond, a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::CMP_I_I_V(cond, a, b_var, out) => {
                let b = get_var(b_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                cmp!(cond, a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::CMP_V_I_V(cond_var, a, b_var, out) => {
                let cond = get_var(cond_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                cmp!(cond, a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::CMP_I_V_V(cond, a_var, b_var, out) => {
                let a = get_var(a_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                cmp!(cond, a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }
            Opcode::CMP_V_V_V(cond_var, a_var, b_var, out) => {
                let cond = get_var(cond_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let a = get_var(a_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();
                let b = get_var(b_var, global_scope, stack, cur_frame, module_frame, global_frame).clone();

                cmp!(cond, a, b, out, global_scope, stack, cur_frame, module_frame, global_frame);
            }

            _ => panic!("unknown instruction {:#04x} at {:#06x}", instr.opcode.to_u8(), instr.index)
        }
        
        // times[instr.opcode.to_u8() as usize] += instr_start.elapsed().as_secs_f64() * 1000f64;
        // counts[instr.opcode.to_u8() as usize] += 1;
        
        if !skip_inc {
            *pc += 1;
        }
        skip_inc = false;

        if *pc < block_start {
            break;
        }
    }
    
    // println!("scope took {:.4}ms", start.elapsed().as_secs_f32() * 1000f32);

    // for x in 0x00..0xff {
    //     if counts[x] > 0 {
    //         println!("{:#04x}: {:.6}ms avg | {:.6}ms total", x, times[x] / counts[x] as f64, times[x]);
    //     }
    // }

    return 0;
}

pub fn exec_scope(scope: &Scope, global_scope: &Scope, stack: &mut Vec<Frame>, cur_frame: usize, pop_stack: bool, pc: &mut usize, module_frame: usize, global_frame: usize) -> i32 {
    let scope_stack_start = stack[cur_frame].stack.len();

    for (_, module) in &scope.modules {
        let retval = exec_scope(&module.scope, &global_scope, stack, module.frame, pop_stack, &mut 0, module.frame, global_frame);
        if retval != 0 {
            return retval;
        }
    }

    let mut i = 0;
    while i < scope.blocks.len() {
        let start;
        if scope.block_starts.len() > i + 1 {
            while *pc > scope.block_starts[i + 1] {
                i += 1;

                if i >= scope.block_starts.len() - 1 {
                    break;
                }
            }
            
            if i >= scope.block_starts.len() {
                break;
            }
            
            start = scope.block_starts[i];
        } else {
            start = scope.block_starts[i];
        }

        let block = &scope.blocks[i];

        let ret = match block {
            Block::CODE(vec) => exec_block(scope, vec, global_scope, stack, cur_frame, pc, start, module_frame, global_frame),
            Block::SCOPE(scope) => {
                *pc += 1;
                exec_scope(&scope, global_scope, stack, cur_frame, pop_stack, &mut 0, module_frame, global_frame)
            }
        };

        let mut skip_inc = false;
        if i > 0 {
            while *pc < scope.block_starts[i] {
                i -= 1;
                skip_inc = true;
                
                if i <= 0 {
                    break;
                }
            }
        }

        if ret != 0 {
            return ret;
        }
        
        if !skip_inc {
            i += 1;
        }
    }

    if pop_stack {
        while stack[cur_frame].stack.len() > scope_stack_start {
            stack[cur_frame].pop();
        }
    }

    return 0;
}

pub fn exec_func(func: &Function, global_scope: &Scope, stack: &mut Vec<Frame>, module_frame: usize, global_frame: usize) -> i32 {
    let len = stack.len();

    stack.push(Frame { vars: HashMap::new(), stack: Vec::new(), allocs: Vec::new() });

    for i in 0..func.arg_names.len() {
        // TODO: argument type checking
        let val = stack[len - 1].pop();
        let index = func.arg_names.len() - 1 - i;
        stack[len].push_var(&func.arg_names[index], func.arg_types[index].clone(), val.val);
    }

    let retval = exec_scope(&func.scope, global_scope, stack, len, true, &mut 0, module_frame, global_frame);

    stack.pop();

    return retval;
}