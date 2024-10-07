use std::collections::HashMap;

use crate::{_type::Types, ffi::call_ffi, frame::Frame, func_exists, function::Function, get_extern, get_func, get_struct, get_var, instruction::Opcode, scope::Scope, set_var, value::Values};

// instruction macros
macro_rules! peek {
    ($val:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        let index;
        match $val.val {
            Values::SIGNED(n) => index = n as usize,
            Values::UNSIGNED(n) => index = n as usize,
            Values::DECIMAL(n) => index = n as usize,
            Values::POINTER(n, _) => index = n as usize,
            _ => panic!("cannot peek using a non-numeral value index"),
        }

        let val = $stack[$cur_frame].stack[index].val.clone();
        set_var($out, &val, $global_scope, $stack, $cur_frame);
    }
}

macro_rules! call {
    ($func:expr, $scope:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        if func_exists($func, $scope, $global_scope) {
            let func = get_func($func, $scope, $global_scope);
            exec_func(func, $global_scope, $stack);
        } else {
            let func = get_extern($func, $scope, $global_scope);
            call_ffi(func, $stack, $cur_frame);
        }
    }
}

macro_rules! add {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        let val = $a.val.add(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame);
    };
}
macro_rules! sub {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        let val = $a.val.sub(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame);
    };
}
macro_rules! mul {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        let val = $a.val.mul(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame);
    };
}
macro_rules! div {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        let val = $a.val.div(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame);
    };
}
macro_rules! modulo {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        let val = $a.val.modulo(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame);
    };
}

macro_rules! compare {
    ($a_val:expr, $b:expr, $op:tt, $pc:expr, $new_pc:expr) => {
        match $b {
            Values::SIGNED(b_val) => {
                if ($a_val as i64) $op b_val {
                    $pc = $new_pc as usize;
                }
            }
            Values::UNSIGNED(b_val) => {
                if ($a_val as u64) $op b_val {
                    $pc = $new_pc as usize;
                }
            }
            Values::DECIMAL(b_val) => {
                if ($a_val as f64) $op b_val {
                    $pc = $new_pc as usize;
                }
            }
            _ => panic!("expected a number for comparison, got {:?}", $b),
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
    ($a:expr, $b:expr, $c:expr, $pc:expr) => {
        let mut new_pc;
        get_pc!($c.val, new_pc);

        new_pc -= 1;

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, !=, $pc, new_pc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, !=, $pc, new_pc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, !=, $pc, new_pc),
            _ => panic!("expected a number for comparison, got {:?}", $a.val)
        }
    }
}
macro_rules! je {
    ($a:expr, $b:expr, $c:expr, $pc:expr) => {
        let mut new_pc;
        get_pc!($c.val, new_pc);

        new_pc -= 1;

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, ==, $pc, new_pc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, ==, $pc, new_pc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, ==, $pc, new_pc),
            _ => panic!("expected a number for comparison, got {:?}", $a.val)
        }
    }
}

macro_rules! jge {
    ($a:expr, $b:expr, $c:expr, $pc:expr) => {
        let mut new_pc;
        get_pc!($c.val, new_pc);

        new_pc -= 1;

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, >=, $pc, new_pc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, >=, $pc, new_pc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, >=, $pc, new_pc),
            _ => panic!("expected a number for comparison, got {:?}", $a.val)
        }
    }
}
macro_rules! jg {
    ($a:expr, $b:expr, $c:expr, $pc:expr) => {
        let mut new_pc;
        get_pc!($c.val, new_pc);

        new_pc -= 1;

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, >, $pc, new_pc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, >, $pc, new_pc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, >, $pc, new_pc),
            _ => panic!("expected a number for comparison, got {:?}", $a.val)
        }
    }
}
macro_rules! jle {
    ($a:expr, $b:expr, $c:expr, $pc:expr) => {
        let mut new_pc;
        get_pc!($c.val, new_pc);

        new_pc -= 1;

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, <=, $pc, new_pc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, <=, $pc, new_pc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, <=, $pc, new_pc),
            _ => panic!("expected a number for comparison, got {:?}", $a.val)
        }
    }
}
macro_rules! jl {
    ($a:expr, $b:expr, $c:expr, $pc:expr) => {
        let mut new_pc;
        get_pc!($c.val, new_pc);

        new_pc -= 1;

        match $a.val {
            Values::SIGNED(a_val) => compare!(a_val, $b.val, <, $pc, new_pc),
            Values::UNSIGNED(a_val) => compare!(a_val, $b.val, <, $pc, new_pc),
            Values::DECIMAL(a_val) => compare!(a_val, $b.val, <, $pc, new_pc),
            _ => panic!("expected a number for comparison, got {:?}", $a.val)
        }
    }
}

macro_rules! mov {
    ($a:expr, $b:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        set_var($b, &$a.val, $global_scope, $stack, $cur_frame);
    }
}

macro_rules! and {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        let val = $a.val.and(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame);
    };
}

macro_rules! or {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        let val = $a.val.or(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame);
    };
}

macro_rules! xor {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        let val = $a.val.xor(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame);
    };
}

macro_rules! not {
    ($a:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        let val = $a.val.not();
        set_var($out, &val, $global_scope, $stack, $cur_frame);
    };
}

macro_rules! lsh {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        let val = $a.val.lsh(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame);
    };
}

macro_rules! rsh {
    ($a:expr, $b:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        let val = $a.val.rsh(&$b.val);
        set_var($out, &val, $global_scope, $stack, $cur_frame);
    };
}

macro_rules! get_type {
    ($typ:expr, $type_var:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $action:expr) => {
        let type_var = get_var($type_var, $global_scope, $stack, $cur_frame);

        match &type_var.val {
            Values::TYPE(t) => $typ = t.clone(),
            _ => panic!("tried to {} with dynamic type stored in variable, but given variable had type {:?}", $action, type_var.typ)
        }
    }
}
macro_rules! get_name {
    ($name:expr, $name_var:expr, $global_scope:expr, $stack:expr, $cur_frame:expr, $action:expr) => {
        let name_var = get_var($name_var, $global_scope, $stack, $cur_frame);

        match &name_var.val {
            Values::NAME(n) => $name = n.clone(),
            _ => panic!("tried to {} variable with dynamic name stored in variable, but given variable had type {:?}", $action, name_var.typ)
        }
    }
}


macro_rules! ref_ {
    ($index:expr, $out_var:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        // ugly line
        let out_var_type = get_var($out_var, $global_scope, $stack, $cur_frame).typ.typ[0].clone();
        match out_var_type {
            Types::POINTER => {
                set_var($out_var, &Values::POINTER($index, 1), $global_scope, $stack, $cur_frame);
            }
            _ => panic!("attempted set a variable with type {:?} to a reference", out_var_type)
        }
    }
}

macro_rules! deref {
    ($ptr:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        let index;
        match $ptr.val {
            Values::POINTER(p, 1) => index = p,
            _ => panic!("attempted to deref non-pointer value")
        }
        
        let val = $stack[0].stack[index].val.clone();
        set_var($out, &val, $global_scope, $stack, $cur_frame);
    }
}

macro_rules! get_usize {
    ($index:expr, $amnt:expr, $action:expr, $type:expr) => {
        $index = match($amnt.val) {
            Values::SIGNED(n) => n as usize,
            Values::UNSIGNED(n) => n as usize,
            Values::DECIMAL(n) => n as usize,
            Values::POINTER(n, _) => n,
            _ => panic!("cannot {} with non-number value as {}", $action, $type),
        };
    }
}

macro_rules! pmov {
    ($val:expr, $ptr:expr, $offset:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        let ptr = get_var($ptr, $global_scope, $stack, $cur_frame);
        let ptr = match(ptr.val) {
            Values::POINTER(n, _) => n,
            _ => panic!("cannot PMOV into a non-pointer variable")
        };

        let offset_index;
        get_usize!(offset_index, $offset, "PMOV", "offset");

        $stack[0].stack[ptr + offset_index].set(&$val.val);
    }
}

macro_rules! alloc {
    ($typ:expr, $amnt:expr, $out:expr, $global_scope:expr, $stack:expr, $cur_frame:expr) => {
        let amnt = match($amnt.val) {
            Values::SIGNED(n) => n as u64,
            Values::UNSIGNED(n) => n,
            Values::DECIMAL(n) => n as u64,
            _ => panic!("cannot allocate with non-number value as count"),
        };

        let index = $stack[0].stack.len();

        $stack[$cur_frame].set_var($out, &Values::POINTER(index, amnt as usize));

        for _ in 0..amnt {
            $stack[0].push_alloc($typ, $out.clone());
        }
    }
}

macro_rules! free_ {
    ($ptr:expr, $amnt:expr, $stack:expr) => {
        let mut index;
        get_usize!(index, $ptr, "free", "pointer");

        let size;
        get_usize!(size, $amnt, "free", "size");

        let start = index;

        // TODO: this loop will get extremely slow with large allocs
        //       replace this with full heap reconstruction, or somehow allow the heap to get fragmented
        for _ in 0..size {
            $stack[0].allocs.remove(start);
            $stack[0].stack.remove(start);
            index += 1;
        }
    }
}

// TODO: make scopes in scopes preserve instruction order
// example:
// ...
// {
//   these instructions should be executed where they are
//   but as it stands they are executed last
// }
// ...
pub fn exec_scope(scope: &Scope, global_scope: &Scope, stack: &mut Vec<Frame>, cur_frame: usize, pop_stack: bool) -> i32 {
    let mut pc = 0;

    // i want to make per-instruction timing toggleable
    // but i also want to do it in a way that doesnt have any performance impact
    // i'll have to figure out a way
    // let mut times: [f64; 256] = [0f64; 256];
    // let mut counts: [u32; 256] = [0; 256];

    let scope_stack_start = stack[cur_frame].stack.len();

    // let start = std::time::Instant::now();
    while pc < scope.instructions.len() {
        let instr = &scope.instructions[pc];

        // let instr_start = std::time::Instant::now();
        match &instr.opcode {
            Opcode::NOP => { // NOP
                // do nothing
            }

            Opcode::PUSH_IMM(val) => { // PUSH [imm]
                stack[cur_frame].push(val.clone());
            }
            Opcode::PUSH_VAR(name) => { // PUSH [var]
                let var = get_var(name, global_scope, stack, cur_frame);

                let val = var.clone();
                stack[cur_frame].push(val);
            }

            Opcode::POP(name) => { // POP [var]
                set_var(name, &stack[cur_frame].pop().val, global_scope, stack, cur_frame);
            }

            Opcode::PEEK_IMM(val, out) => { // PEEK [imm] [var]
                peek!(val, out, global_scope, stack, cur_frame);
            }
            Opcode::PEEK_VAR(val_var, out) => { // PEEK [var] [var]
                let val = get_var(val_var, global_scope, stack, cur_frame);

                peek!(val, out, global_scope, stack, cur_frame);
            }

            Opcode::CALL_FUNC(func) => { // CALL [func]
                call!(func, scope, global_scope, stack, cur_frame);
            }
            Opcode::CALL_VAR(func_var) => { // CALL [var]
                let func_var = get_var(func_var, global_scope, stack, cur_frame);

                let func;
                match &func_var.val {
                    Values::NAME(n) => func = n,
                    _ => panic!("tried to call function with name stored in variable, but given variable had type {:?}", func_var.typ)
                }

                call!(func, scope, global_scope, stack, cur_frame);
            }

            Opcode::ADD_I_I(a, b, out) => { // ADD [imm] [imm] [var]
                add!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::ADD_V_I(a_name, b, out) => { // ADD [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();

                add!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::ADD_I_V(a, b_name, out) => { // ADD [imm] [var] [var]                
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                add!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::ADD_V_V(a_name, b_name, out) => { // ADD [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                add!(a, b, out, global_scope, stack, cur_frame);
            }

            Opcode::SUB_I_I(a, b, out) => { // SUB [imm] [imm] [var]
                sub!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::SUB_V_I(a_name, b, out) => { // SUB [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();

                sub!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::SUB_I_V(a, b_name, out) => { // SUB [imm] [var] [var]                
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                sub!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::SUB_V_V(a_name, b_name, out) => { // SUB [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                sub!(a, b, out, global_scope, stack, cur_frame);
            }

            Opcode::MUL_I_I(a, b, out) => { // MUL [imm] [imm] [var]
                mul!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::MUL_V_I(a_name, b, out) => { // MUL [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();

                mul!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::MUL_I_V(a, b_name, out) => { // MUL [imm] [var] [var]                
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                mul!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::MUL_V_V(a_name, b_name, out) => { // MUL [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                mul!(a, b, out, global_scope, stack, cur_frame);
            }

            Opcode::DIV_I_I(a, b, out) => { // DIV [imm] [imm] [var]
                div!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::DIV_V_I(a_name, b, out) => { // DIV [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();

                div!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::DIV_I_V(a, b_name, out) => { // DIV [imm] [var] [var]                
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                div!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::DIV_V_V(a_name, b_name, out) => { // DIV [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                div!(a, b, out, global_scope, stack, cur_frame);
            }

            Opcode::JMP_IMM(new_pc_val) => { // JMP [imm]
                let new_pc: usize;
                get_pc!(new_pc_val.val.clone(), new_pc);

                pc = new_pc - 1;
            }
            Opcode::JMP_VAR(new_pc_name) => { // JMP [var]
                let new_pc_var = get_var(new_pc_name, global_scope, stack, cur_frame).val.clone();
                let new_pc: usize;
                get_pc!(new_pc_var, new_pc);

                pc = new_pc - 1;
            }

            Opcode::JNE_I_I_I(a, b, c) => { // JNE [imm] [imm] [imm]
                jne!(a, b, c, pc);
            }
            Opcode::JNE_V_I_I(a_name, b, c) => { // JNE [var] [imm] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();

                jne!(a, b, c, pc);
            }
            Opcode::JNE_I_V_I(a, b_name, c) => { // JNE [imm] [imm] [imm]
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                jne!(a, b, c, pc);
            }
            Opcode::JNE_V_V_I(a_name, b_name, c) => { // JNE [var] [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                jne!(a, b, c, pc);
            }
            Opcode::JNE_I_I_V(a, b, c_name) => { // JNE [imm] [imm] [var]
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jne!(a, b, c, pc);
            }
            Opcode::JNE_V_I_V(a_name, b, c_name) => { // JNE [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jne!(a, b, c, pc);
            }
            Opcode::JNE_I_V_V(a, b_name, c_name) => { // JNE [imm] [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jne!(a, b, c, pc);
            }
            Opcode::JNE_V_V_V(a_name, b_name, c_name) => { // JNE [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jne!(a, b, c, pc);
            }

            Opcode::JE_I_I_I(a, b, c) => { // JE [imm] [imm] [imm]
                je!(a, b, c, pc);
            }
            Opcode::JE_V_I_I(a_name, b, c) => { // JE [var] [imm] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();

                je!(a, b, c, pc);
            }
            Opcode::JE_I_V_I(a, b_name, c) => { // JE [imm] [imm] [imm]
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                je!(a, b, c, pc);
            }
            Opcode::JE_V_V_I(a_name, b_name, c) => { // JE [var] [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                je!(a, b, c, pc);
            }
            Opcode::JE_I_I_V(a, b, c_name) => { // JE [imm] [imm] [var]
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                je!(a, b, c, pc);
            }
            Opcode::JE_V_I_V(a_name, b, c_name) => { // JE [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                je!(a, b, c, pc);
            }
            Opcode::JE_I_V_V(a, b_name, c_name) => { // JE [imm] [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                je!(a, b, c, pc);
            }
            Opcode::JE_V_V_V(a_name, b_name, c_name) => { // JE [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                je!(a, b, c, pc);
            }

            Opcode::JGE_I_I_I(a, b, c) => { // JGE [imm] [imm] [imm]
                jge!(a, b, c, pc);
            }
            Opcode::JGE_V_I_I(a_name, b, c) => { // JGE [var] [imm] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();

                jge!(a, b, c, pc);
            }
            Opcode::JGE_I_V_I(a, b_name, c) => { // JGE [imm] [imm] [imm]
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                jge!(a, b, c, pc);
            }
            Opcode::JGE_V_V_I(a_name, b_name, c) => { // JGE [var] [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                jge!(a, b, c, pc);
            }
            Opcode::JGE_I_I_V(a, b, c_name) => { // JGE [imm] [imm] [var]
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jge!(a, b, c, pc);
            }
            Opcode::JGE_V_I_V(a_name, b, c_name) => { // JGE [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jge!(a, b, c, pc);
            }
            Opcode::JGE_I_V_V(a, b_name, c_name) => { // JGE [imm] [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jge!(a, b, c, pc);
            }
            Opcode::JGE_V_V_V(a_name, b_name, c_name) => { // JGE [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jge!(a, b, c, pc);
            }

            Opcode::JG_I_I_I(a, b, c) => { // JG [imm] [imm] [imm]
                jg!(a, b, c, pc);
            }
            Opcode::JG_V_I_I(a_name, b, c) => { // JG [var] [imm] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();

                jg!(a, b, c, pc);
            }
            Opcode::JG_I_V_I(a, b_name, c) => { // JG [imm] [imm] [imm]
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                jg!(a, b, c, pc);
            }
            Opcode::JG_V_V_I(a_name, b_name, c) => { // JG [var] [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                jg!(a, b, c, pc);
            }
            Opcode::JG_I_I_V(a, b, c_name) => { // JG [imm] [imm] [var]
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jg!(a, b, c, pc);
            }
            Opcode::JG_V_I_V(a_name, b, c_name) => { // JG [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jg!(a, b, c, pc);
            }
            Opcode::JG_I_V_V(a, b_name, c_name) => { // JG [imm] [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jg!(a, b, c, pc);
            }
            Opcode::JG_V_V_V(a_name, b_name, c_name) => { // JG [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jg!(a, b, c, pc);
            }

            Opcode::JLE_I_I_I(a, b, c) => { // JLE [imm] [imm] [imm]
                jle!(a, b, c, pc);
            }
            Opcode::JLE_V_I_I(a_name, b, c) => { // JLE [var] [imm] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();

                jle!(a, b, c, pc);
            }
            Opcode::JLE_I_V_I(a, b_name, c) => { // JLE [imm] [imm] [imm]
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                jle!(a, b, c, pc);
            }
            Opcode::JLE_V_V_I(a_name, b_name, c) => { // JLE [var] [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                jle!(a, b, c, pc);
            }
            Opcode::JLE_I_I_V(a, b, c_name) => { // JLE [imm] [imm] [var]
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jle!(a, b, c, pc);
            }
            Opcode::JLE_V_I_V(a_name, b, c_name) => { // JLE [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jle!(a, b, c, pc);
            }
            Opcode::JLE_I_V_V(a, b_name, c_name) => { // JLE [imm] [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jle!(a, b, c, pc);
            }
            Opcode::JLE_V_V_V(a_name, b_name, c_name) => { // JLE [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jle!(a, b, c, pc);
            }

            Opcode::JL_I_I_I(a, b, c) => { // JL [imm] [imm] [imm]
                jl!(a, b, c, pc);
            }
            Opcode::JL_V_I_I(a_name, b, c) => { // JL [var] [imm] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();

                jl!(a, b, c, pc);
            }
            Opcode::JL_I_V_I(a, b_name, c) => { // JL [imm] [imm] [imm]
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                jl!(a, b, c, pc);
            }
            Opcode::JL_V_V_I(a_name, b_name, c) => { // JL [var] [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                jl!(a, b, c, pc);
            }
            Opcode::JL_I_I_V(a, b, c_name) => { // JL [imm] [imm] [var]
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jl!(a, b, c, pc);
            }
            Opcode::JL_V_I_V(a_name, b, c_name) => { // JL [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jl!(a, b, c, pc);
            }
            Opcode::JL_I_V_V(a, b_name, c_name) => { // JL [imm] [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jl!(a, b, c, pc);
            }
            Opcode::JL_V_V_V(a_name, b_name, c_name) => { // JL [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();
                let c = get_var(c_name, global_scope, stack, cur_frame).clone();

                jl!(a, b, c, pc);
            }

            Opcode::MOV_I_V(a, b) => { // MOV [imm] [var]
                mov!(a, b, global_scope, stack, cur_frame);
            }
            Opcode::MOV_V_V(a_name, b) => { // MOV [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();

                mov!(a, b, global_scope, stack, cur_frame);
            }
            Opcode::MOV_VV_V(a_var, b) => { // MOV [var var] [var]
                let a_name;
                get_name!(a_name, a_var, global_scope, stack, cur_frame, "access");

                let a = get_var(&a_name, global_scope, stack, cur_frame).clone();

                mov!(a, b, global_scope, stack, cur_frame);
            }
            Opcode::MOV_I_VV(a, b_var) => { // MOV [imm] [var var]
                let b;
                get_name!(b, b_var, global_scope, stack, cur_frame, "set");

                mov!(a, &b, global_scope, stack, cur_frame);
            }
            Opcode::MOV_V_VV(a_name, b_var) => { // MOV [var] [var var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();

                let b;
                get_name!(b, b_var, global_scope, stack, cur_frame, "set");

                mov!(a, &b, global_scope, stack, cur_frame);
            }
            Opcode::MOV_VV_VV(a_var, b_var) => { // MOV [var var] [var var]
                let a_name;
                get_name!(a_name, a_var, global_scope, stack, cur_frame, "access");

                let a = get_var(&a_name, global_scope, stack, cur_frame).clone();

                let b;
                get_name!(b, b_var, global_scope, stack, cur_frame, "set");

                mov!(a, &b, global_scope, stack, cur_frame);
            }

            Opcode::AND_I_I(a, b, out) => { // AND [imm] [imm]
                and!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::AND_V_I(a_name, b, out) => { // AND [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                
                and!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::AND_I_V(a, b_name, out) => { // AND [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                and!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::AND_V_V(a_name, b_name, out) => { // AND [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                and!(a, b, out, global_scope, stack, cur_frame);
            }

            Opcode::OR_I_I(a, b, out) => { // OR [imm] [imm]
                or!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::OR_V_I(a_name, b, out) => { // OR [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                
                or!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::OR_I_V(a, b_name, out) => { // OR [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                or!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::OR_V_V(a_name, b_name, out) => { // OR [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                or!(a, b, out, global_scope, stack, cur_frame);
            }

            Opcode::XOR_I_I(a, b, out) => { // XOR [imm] [imm]
                xor!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::XOR_V_I(a_name, b, out) => { // XOR [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                
                xor!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::XOR_I_V(a, b_name, out) => { // XOR [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                xor!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::XOR_V_V(a_name, b_name, out) => { // XOR [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                xor!(a, b, out, global_scope, stack, cur_frame);
            }

            Opcode::NOT_IMM(a, out) => { // NOT [imm]
                not!(a, out, global_scope, stack, cur_frame);
            }
            Opcode::NOT_VAR(a_name, out) => { // NOT [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                
                not!(a, out, global_scope, stack, cur_frame);
            }

            Opcode::LSH_I_I(a, b, out) => { // LSH [imm] [imm]
                lsh!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::LSH_V_I(a_name, b, out) => { // LSH [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                
                lsh!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::LSH_I_V(a, b_name, out) => { // LSH [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                lsh!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::LSH_V_V(a_name, b_name, out) => { // LSH [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                lsh!(a, b, out, global_scope, stack, cur_frame);
            }

            Opcode::RSH_I_I(a, b, out) => { // RSH [imm] [imm]
                rsh!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::RSH_V_I(a_name, b, out) => { // RSH [var] [imm]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                
                rsh!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::RSH_I_V(a, b_name, out) => { // RSH [imm] [var]
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                rsh!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::RSH_V_V(a_name, b_name, out) => { // RSH [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                rsh!(a, b, out, global_scope, stack, cur_frame);
            }
            
            Opcode::VAR_TYPE_NAME(typ, name) => { // VAR [type] [name]
                stack[cur_frame].create_var(name.clone(), typ.clone());
            }
            Opcode::VAR_VAR_NAME(type_var, name) => { // VAR [var] [name]
                let typ;
                get_type!(typ, type_var, global_scope, stack, cur_frame, "create variable");
                
                stack[cur_frame].create_var(name.clone(), typ);
            }
            Opcode::VAR_TYPE_VAR(typ, name_var) => { // VAR [type] [var]
                let name;
                get_name!(name, name_var, global_scope, stack, cur_frame, "create");

                stack[cur_frame].create_var(name, typ.clone())
            }
            Opcode::VAR_VAR_VAR(type_var, name_var) => { // VAR [var] [var]
                let typ;
                get_type!(typ, type_var, global_scope, stack, cur_frame, "create variable");

                let name;
                get_name!(name, name_var, global_scope, stack, cur_frame, "create");

                stack[cur_frame].create_var(name, typ);
            }

            // TODO: return type checking
            Opcode::RET => { // RET
                break;
            }
            Opcode::RET_IMM(v) => { // RET [imm]
                if cur_frame != 0 {
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
                let v = get_var(var, global_scope, stack, cur_frame).clone();
                
                if cur_frame != 0 {
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
                let index = stack[0].stack.len();

                stack[0].push(val.clone());

                ref_!(index, out_var, global_scope, stack, cur_frame);
            }
            Opcode::REF_VAR(var, out_var) => {
                let index = stack[0].stack.len();

                // we only need to move the variable to the heap if it isnt already on the heap
                // TODO: figure out a way to change the name of the created variable on the heap
                //       to remove the possibility of name collisions
                //       
                //       if you have a variable with a certain name and you REF it, if there
                //       is a global variable with the same name it wil overwrite it
                if !stack[0].vars.contains_key(var) {
                    if stack[cur_frame].vars.contains_key(var) {
                        let orig_var = stack[cur_frame].get_var(var).clone();

                        stack[0].push_var(var, orig_var.typ, orig_var.val);
                    } else {
                        panic!("attempted to create a reference to a variable that doesnt exist");
                    }
                }
                
                ref_!(index, out_var, global_scope, stack, cur_frame);
            }

            Opcode::DEREF_IMM(ptr, out) => {
                deref!(ptr, out, global_scope, stack, cur_frame);
            }
            Opcode::DEREF_VAR(ptr_var, out) => {
                let ptr = get_var(ptr_var, global_scope, stack, cur_frame);

                deref!(ptr, out, global_scope, stack, cur_frame);
            }

            Opcode::INST_NAME(struct_name, out) => { // INST [name] [var]
                let frame = &stack[cur_frame];
                let start_index = frame.stack.len();

                let struct_type = get_struct(struct_name, global_scope);

                let strct = Values::STRUCT(struct_name.clone(), start_index);

                set_var(out, &strct, global_scope, stack, cur_frame);

                for i in 0..struct_type.var_types.len() {
                    stack[cur_frame].push_type(&struct_type.var_types[i]);
                }
            }

            Opcode::MOD_I_I(a, b, out) => { // MOD [imm] [imm] [var]
                modulo!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::MOD_V_I(a_name, b, out) => { // MOD [var] [imm] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();

                modulo!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::MOD_I_V(a, b_name, out) => { // MOD [imm] [var] [var]                
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                modulo!(a, b, out, global_scope, stack, cur_frame);
            }
            Opcode::MOD_V_V(a_name, b_name, out) => { // MOD [var] [var] [var]
                let a = get_var(a_name, global_scope, stack, cur_frame).clone();
                let b = get_var(b_name, global_scope, stack, cur_frame).clone();

                modulo!(a, b, out, global_scope, stack, cur_frame);
            }

            Opcode::PMOV_IMM_IMM(val, ptr, offset) => {
                pmov!(val, ptr, offset, global_scope, stack, cur_frame);
            }
            Opcode::PMOV_VAR_IMM(val_var, ptr, offset) => {
                let val = get_var(val_var, global_scope, stack, cur_frame).clone();

                pmov!(val, ptr, offset, global_scope, stack, cur_frame);
            }
            Opcode::PMOV_IMM_VAR(val, ptr, offset_var) => {
                let offset = get_var(offset_var, global_scope, stack, cur_frame).clone();

                pmov!(val, ptr, offset, global_scope, stack, cur_frame);
            }
            Opcode::PMOV_VAR_VAR(val_var, ptr, offset_var) => {
                let offset = get_var(offset_var, global_scope, stack, cur_frame).clone();
                let val = get_var(val_var, global_scope, stack, cur_frame).clone();
                
                pmov!(val, ptr, offset, global_scope, stack, cur_frame);
            }

            Opcode::ALLOC_TYPE_IMM(typ, amnt, out) => {
                alloc!(typ, amnt, out, global_scope, stack, cur_frame);
            }
            Opcode::ALLOC_VAR_IMM(type_var, amnt, out) => {
                let typ;
                get_type!(typ, type_var, global_scope, stack, cur_frame, "allocate");

                alloc!(&typ, amnt, out, global_scope, stack, cur_frame);
            }
            Opcode::ALLOC_TYPE_VAR(typ, amnt_var, out) => {
                let amnt = get_var(amnt_var, global_scope, stack, cur_frame);

                alloc!(typ, amnt, out, global_scope, stack, cur_frame);
            }
            Opcode::ALLOC_VAR_VAR(type_var, amnt_var, out) => {
                let typ;
                get_type!(typ, type_var, global_scope, stack, cur_frame, "allocate");

                let amnt = get_var(amnt_var, global_scope, stack, cur_frame);

                alloc!(&typ, amnt, out, global_scope, stack, cur_frame);
            }

            Opcode::FREE_VAR(ptr) => {
                let mut index = *stack[0].vars.get(ptr).unwrap_or_else(|| panic!("attempted to free non-existant pointer {}", ptr));
                let start = index;

                stack[0].vars.remove(ptr);

                // TODO: this loop will get extremely slow with large allocs
                //       replace this with full heap reconstruction, or somehow allow the heap to get fragmented
                while &stack[0].allocs[index] == ptr {
                    stack[0].allocs.remove(start);
                    stack[0].stack.remove(start);
                    index += 1;
                }
            }
            Opcode::FREE_IMM_IMM(ptr, amnt) => {
                free_!(ptr, amnt, stack);
            }
            Opcode::FREE_VAR_IMM(ptr_var, amnt) => {
                let ptr = get_var(ptr_var, global_scope, stack, cur_frame).clone();

                free_!(ptr, amnt, stack);
            }
            Opcode::FREE_IMM_VAR(ptr, amnt_var) => {
                let amnt = get_var(amnt_var, global_scope, stack, cur_frame).clone();

                free_!(ptr, amnt, stack);
            }
            Opcode::FREE_VAR_VAR(ptr_var, amnt_var) => {
                let ptr = get_var(ptr_var, global_scope, stack, cur_frame).clone();
                let amnt = get_var(amnt_var, global_scope, stack, cur_frame).clone();

                free_!(ptr, amnt, stack);
            }

            _ => panic!("unknown instruction {:#04x} at {:#06x}", instr.opcode.to_u8(), instr.index)
        }
        
        // times[instr.opcode.to_u8() as usize] += instr_start.elapsed().as_secs_f64() * 1000f64;
        // counts[instr.opcode.to_u8() as usize] += 1;
        
        pc += 1;
    }

    // clear everything from the stack created by the scope
    if pop_stack {
        while stack[cur_frame].stack.len() > scope_stack_start {
            stack[cur_frame].pop();
        }
    }

    return 0;
    
    // println!("scope took {:.4}ms", start.elapsed().as_secs_f32() * 1000f32);

    // for x in 0x00..0xff {
    //     if counts[x] > 0 {
    //         println!("{:#04x}: {:.6}ms avg | {:.6}ms total", x, times[x] / counts[x] as f64, times[x]);
    //     }
    // }
}

pub fn exec_func(func: &Function, global_scope: &Scope, stack: &mut Vec<Frame>) -> i32 {
    stack.push(Frame { vars: HashMap::new(), stack: Vec::new(), allocs: Vec::new() });

    let len = stack.len();

    for i in 0..func.arg_names.len() {
        // TODO: argument type checking
        let val = stack[len - 2].pop();
        let index = func.arg_names.len() - 1 - i;
        stack[len - 1].push_var(&func.arg_names[index], func.arg_types[index].clone(), val.val);
    }

    let retval = exec_scope(&func.scope, global_scope, stack, len - 1, true);

    stack.pop();

    return retval;
}