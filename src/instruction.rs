use crate::{_type::Type, value::Value};

#[derive(Debug, Clone)]
#[repr(u8)]
#[allow(non_camel_case_types)]
pub enum Opcode { // very large enum 
    NOP                                   = 0x00,
    PUSH_IMM(Value)                       = 0x01,
    PUSH_VAR(String)                      = 0x02,
    POP(String)                           = 0x03,
    PEEK_IMM(Value, String)               = 0x04,
    PEEK_VAR(String, String)              = 0x05,
    CALL_FUNC(String)                     = 0x06,
    CALL_VAR(String)                      = 0x07,
    ADD_I_I(Value, Value, String)         = 0x08,
    ADD_V_I(String, Value, String)        = 0x09,
    ADD_I_V(Value, String, String)        = 0x0A,
    ADD_V_V(String, String, String)       = 0x0B,
    SUB_I_I(Value, Value, String)         = 0x0C,
    SUB_V_I(String, Value, String)        = 0x0D,
    SUB_I_V(Value, String, String)        = 0x0E,
    SUB_V_V(String, String, String)       = 0x0F,
    MUL_I_I(Value, Value, String)         = 0x10,
    MUL_V_I(String, Value, String)        = 0x11,
    MUL_I_V(Value, String, String)        = 0x12,
    MUL_V_V(String, String, String)       = 0x13,
    DIV_I_I(Value, Value, String)         = 0x14,
    DIV_V_I(String, Value, String)        = 0x15,
    DIV_I_V(Value, String, String)        = 0x16,
    DIV_V_V(String, String, String)       = 0x17,
    JMP_IMM(Value)                        = 0x18,
    JMP_VAR(String)                       = 0x19,
    JNE_I_I_I(Value, Value, Value)        = 0x1A,
    JNE_V_I_I(String, Value, Value)       = 0x1B,
    JNE_I_V_I(Value, String, Value)       = 0x1C,
    JNE_V_V_I(String, String, Value)      = 0x1D,
    JNE_I_I_V(Value, Value, String)       = 0x1E,
    JNE_V_I_V(String, Value, String)      = 0x1F,
    JNE_I_V_V(Value, String, String)      = 0x20,
    JNE_V_V_V(String, String, String)     = 0x21,
    JE_I_I_I(Value, Value, Value)         = 0x22,
    JE_V_I_I(String, Value, Value)        = 0x23,
    JE_I_V_I(Value, String, Value)        = 0x24,
    JE_V_V_I(String, String, Value)       = 0x25,
    JE_I_I_V(Value, Value, String)        = 0x26,
    JE_V_I_V(String, Value, String)       = 0x27,
    JE_I_V_V(Value, String, String)       = 0x28,
    JE_V_V_V(String, String, String)      = 0x29,
    JGE_I_I_I(Value, Value, Value)        = 0x2A,
    JGE_V_I_I(String, Value, Value)       = 0x2B,
    JGE_I_V_I(Value, String, Value)       = 0x2C,
    JGE_V_V_I(String, String, Value)      = 0x2D,
    JGE_I_I_V(Value, Value, String)       = 0x2E,
    JGE_V_I_V(String, Value, String)      = 0x2F,
    JGE_I_V_V(Value, String, String)      = 0x30,
    JGE_V_V_V(String, String, String)     = 0x31,
    JG_I_I_I(Value, Value, Value)         = 0x32,
    JG_V_I_I(String, Value, Value)        = 0x33,
    JG_I_V_I(Value, String, Value)        = 0x34,
    JG_V_V_I(String, String, Value)       = 0x35,
    JG_I_I_V(Value, Value, String)        = 0x36,
    JG_V_I_V(String, Value, String)       = 0x37,
    JG_I_V_V(Value, String, String)       = 0x38,
    JG_V_V_V(String, String, String)      = 0x39,
    JLE_I_I_I(Value, Value, Value)        = 0x3A,
    JLE_V_I_I(String, Value, Value)       = 0x3B,
    JLE_I_V_I(Value, String, Value)       = 0x3C,
    JLE_V_V_I(String, String, Value)      = 0x3D,
    JLE_I_I_V(Value, Value, String)       = 0x3E,
    JLE_V_I_V(String, Value, String)      = 0x3F,
    JLE_I_V_V(Value, String, String)      = 0x40,
    JLE_V_V_V(String, String, String)     = 0x41,
    JL_I_I_I(Value, Value, Value)         = 0x42,
    JL_V_I_I(String, Value, Value)        = 0x43,
    JL_I_V_I(Value, String, Value)        = 0x44,
    JL_V_V_I(String, String, Value)       = 0x45,
    JL_I_I_V(Value, Value, String)        = 0x46,
    JL_V_I_V(String, Value, String)       = 0x47,
    JL_I_V_V(Value, String, String)       = 0x48,
    JL_V_V_V(String, String, String)      = 0x49,
    MOV_I_V(Value, String)                = 0x4A,
    MOV_V_V(String, String)               = 0x4B,
    MOV_VV_V(String, String)              = 0x4C,
    MOV_I_VV(Value, String)               = 0x4D,
    MOV_V_VV(String, String)              = 0x4E,
    MOV_VV_VV(String, String)             = 0x4F,
    AND_I_I(Value, Value, String)         = 0x50,
    AND_V_I(String, Value, String)        = 0x51,
    AND_I_V(Value, String, String)        = 0x52,
    AND_V_V(String, String, String)       = 0x53,
    OR_I_I(Value, Value, String)          = 0x54,
    OR_V_I(String, Value, String)         = 0x55,
    OR_I_V(Value, String, String)         = 0x56,
    OR_V_V(String, String, String)        = 0x57,
    XOR_I_I(Value, Value, String)         = 0x58,
    XOR_V_I(String, Value, String)        = 0x59,
    XOR_I_V(Value, String, String)        = 0x5A,
    XOR_V_V(String, String, String)       = 0x5B,
    NOT_IMM(Value)                        = 0x5C,
    NOT_VAR(String)                       = 0x5D,
    LSH_I_I(Value, Value, String)         = 0x5E,
    LSH_V_I(String, Value, String)        = 0x5F,
    LSH_I_V(Value, String, String)        = 0x60,
    LSH_V_V(String, String, String)       = 0x61,
    RSH_I_I(Value, Value, String)         = 0x62,
    RSH_V_I(String, Value, String)        = 0x63,
    RSH_I_V(Value, String, String)        = 0x64,
    RSH_V_V(String, String, String)       = 0x65,
    VAR_TYPE_NAME(Type, String)           = 0x66,
    VAR_VAR_NAME(String, String)          = 0x67,
    VAR_TYPE_VAR(Type, String)            = 0x68,
    VAR_VAR_VAR(String, String)           = 0x69,
    RET                                   = 0x6A,
    RET_IMM(Value)                        = 0x6B,
    RET_VAR(String)                       = 0x6C,
    DEREF_IMM(Value, String)              = 0x6D,
    DEREF_VAR(String, String)             = 0x6E,
    REF_IMM(Value, String)                = 0x6F,
    REF_VAR(String, String)               = 0x70,
    INST_NAME(String)                     = 0x71,
    INST_VAR(String)                      = 0x72,
    MOD_I_I(Value, Value, String)         = 0x73,
    MOD_V_I(String, Value, String)        = 0x74,
    MOD_I_V(Value, String, String)        = 0x75,
    MOD_V_V(String, String, String)       = 0x76,
    PMOV_IMM_IMM(Value, String, Value)    = 0x77,
    PMOV_VAR_IMM(String, String, Value)   = 0x78,
    PMOV_IMM_VAR(Value, String, String)   = 0x79,
    PMOV_VAR_VAR(String, String, String)  = 0x7A,
    ALLOC_TYPE_IMM(Type, Value, String)   = 0x7B,
    ALLOC_VAR_IMM(String, Value, String)  = 0x7C,
    ALLOC_TYPE_VAR(Type, String, String)  = 0x7D,
    ALLOC_VAR_VAR(String, String, String) = 0x7E,
    FREE_VAR(String)                      = 0x7F,
    FREE_IMM_IMM(Value, Value)            = 0x80,
    FREE_VAR_IMM(String, Value)           = 0x81,
    FREE_IMM_VAR(Value, String)           = 0x82,
    FREE_VAR_VAR(String, String)          = 0x83,
}

impl Opcode {
    pub fn to_u8(self: &Opcode) -> u8 {
        match self {
            Opcode::NOP                     => 0x00,
            Opcode::PUSH_IMM(_)             => 0x01,
            Opcode::PUSH_VAR(_)             => 0x02,
            Opcode::POP(_)                  => 0x03,
            Opcode::PEEK_IMM(_, _)          => 0x04,
            Opcode::PEEK_VAR(_, _)          => 0x05,
            Opcode::CALL_FUNC(_)            => 0x06,
            Opcode::CALL_VAR(_)             => 0x07,
            Opcode::ADD_I_I(_, _, _)        => 0x08,
            Opcode::ADD_V_I(_, _, _)        => 0x09,
            Opcode::ADD_I_V(_, _, _)        => 0x0A,
            Opcode::ADD_V_V(_, _, _)        => 0x0B,
            Opcode::SUB_I_I(_, _, _)        => 0x0C,
            Opcode::SUB_V_I(_, _, _)        => 0x0D,
            Opcode::SUB_I_V(_, _, _)        => 0x0E,
            Opcode::SUB_V_V(_, _, _)        => 0x0F,
            Opcode::MUL_I_I(_, _, _)        => 0x10,
            Opcode::MUL_V_I(_, _, _)        => 0x11,
            Opcode::MUL_I_V(_, _, _)        => 0x12,
            Opcode::MUL_V_V(_, _, _)        => 0x13,
            Opcode::DIV_I_I(_, _, _)        => 0x14,
            Opcode::DIV_V_I(_, _, _)        => 0x15,
            Opcode::DIV_I_V(_, _, _)        => 0x16,
            Opcode::DIV_V_V(_, _, _)        => 0x17,
            Opcode::JMP_IMM(_)              => 0x18,
            Opcode::JMP_VAR(_)              => 0x19,
            Opcode::JNE_I_I_I(_, _, _)      => 0x1A,
            Opcode::JNE_V_I_I(_, _, _)      => 0x1B,
            Opcode::JNE_I_V_I(_, _, _)      => 0x1C,
            Opcode::JNE_V_V_I(_, _, _)      => 0x1D,
            Opcode::JNE_I_I_V(_, _, _)      => 0x1E,
            Opcode::JNE_V_I_V(_, _, _)      => 0x1F,
            Opcode::JNE_I_V_V(_, _, _)      => 0x20,
            Opcode::JNE_V_V_V(_, _, _)      => 0x21,
            Opcode::JE_I_I_I(_, _, _)       => 0x22,
            Opcode::JE_V_I_I(_, _, _)       => 0x23,
            Opcode::JE_I_V_I(_, _, _)       => 0x24,
            Opcode::JE_V_V_I(_, _, _)       => 0x25,
            Opcode::JE_I_I_V(_, _, _)       => 0x26,
            Opcode::JE_V_I_V(_, _, _)       => 0x27,
            Opcode::JE_I_V_V(_, _, _)       => 0x28,
            Opcode::JE_V_V_V(_, _, _)       => 0x29,
            Opcode::JGE_I_I_I(_, _, _)      => 0x2A,
            Opcode::JGE_V_I_I(_, _, _)      => 0x2B,
            Opcode::JGE_I_V_I(_, _, _)      => 0x2C,
            Opcode::JGE_V_V_I(_, _, _)      => 0x2D,
            Opcode::JGE_I_I_V(_, _, _)      => 0x2E,
            Opcode::JGE_V_I_V(_, _, _)      => 0x2F,
            Opcode::JGE_I_V_V(_, _, _)      => 0x30,
            Opcode::JGE_V_V_V(_, _, _)      => 0x31,
            Opcode::JG_I_I_I(_, _, _)       => 0x32,
            Opcode::JG_V_I_I(_, _, _)       => 0x33,
            Opcode::JG_I_V_I(_, _, _)       => 0x34,
            Opcode::JG_V_V_I(_, _, _)       => 0x35,
            Opcode::JG_I_I_V(_, _, _)       => 0x36,
            Opcode::JG_V_I_V(_, _, _)       => 0x37,
            Opcode::JG_I_V_V(_, _, _)       => 0x38,
            Opcode::JG_V_V_V(_, _, _)       => 0x39,
            Opcode::JLE_I_I_I(_, _, _)      => 0x3A,
            Opcode::JLE_V_I_I(_, _, _)      => 0x3B,
            Opcode::JLE_I_V_I(_, _, _)      => 0x3C,
            Opcode::JLE_V_V_I(_, _, _)      => 0x3D,
            Opcode::JLE_I_I_V(_, _, _)      => 0x3E,
            Opcode::JLE_V_I_V(_, _, _)      => 0x3F,
            Opcode::JLE_I_V_V(_, _, _)      => 0x40,
            Opcode::JLE_V_V_V(_, _, _)      => 0x41,
            Opcode::JL_I_I_I(_, _, _)       => 0x42,
            Opcode::JL_V_I_I(_, _, _)       => 0x43,
            Opcode::JL_I_V_I(_, _, _)       => 0x44,
            Opcode::JL_V_V_I(_, _, _)       => 0x45,
            Opcode::JL_I_I_V(_, _, _)       => 0x46,
            Opcode::JL_V_I_V(_, _, _)       => 0x47,
            Opcode::JL_I_V_V(_, _, _)       => 0x48,
            Opcode::JL_V_V_V(_, _, _)       => 0x49,
            Opcode::MOV_I_V(_, _)           => 0x4A,
            Opcode::MOV_V_V(_, _)           => 0x4B,
            Opcode::MOV_VV_V(_, _)          => 0x4C,
            Opcode::MOV_I_VV(_, _)          => 0x4D,
            Opcode::MOV_V_VV(_, _)          => 0x4E,
            Opcode::MOV_VV_VV(_, _)         => 0x4F,
            Opcode::AND_I_I(_, _, _)        => 0x50,
            Opcode::AND_V_I(_, _, _)        => 0x51,
            Opcode::AND_I_V(_, _, _)        => 0x52,
            Opcode::AND_V_V(_, _, _)        => 0x53,
            Opcode::OR_I_I(_, _, _)         => 0x54,
            Opcode::OR_V_I(_, _, _)         => 0x55,
            Opcode::OR_I_V(_, _, _)         => 0x56,
            Opcode::OR_V_V(_, _, _)         => 0x57,
            Opcode::XOR_I_I(_, _, _)        => 0x58,
            Opcode::XOR_V_I(_, _, _)        => 0x59,
            Opcode::XOR_I_V(_, _, _)        => 0x5A,
            Opcode::XOR_V_V(_, _, _)        => 0x5B,
            Opcode::NOT_IMM(_)              => 0x5C,
            Opcode::NOT_VAR(_)              => 0x5D,
            Opcode::LSH_I_I(_, _, _)        => 0x5E,
            Opcode::LSH_V_I(_, _, _)        => 0x5F,
            Opcode::LSH_I_V(_, _, _)        => 0x60,
            Opcode::LSH_V_V(_, _, _)        => 0x61,
            Opcode::RSH_I_I(_, _, _)        => 0x62,
            Opcode::RSH_V_I(_, _, _)        => 0x63,
            Opcode::RSH_I_V(_, _, _)        => 0x64,
            Opcode::RSH_V_V(_, _, _)        => 0x65,
            Opcode::VAR_TYPE_NAME(_, _)     => 0x66,
            Opcode::VAR_VAR_NAME(_, _)      => 0x67,
            Opcode::VAR_TYPE_VAR(_, _)      => 0x68,
            Opcode::VAR_VAR_VAR(_, _)       => 0x69,
            Opcode::RET                     => 0x6A,
            Opcode::RET_IMM(_)              => 0x6B,
            Opcode::RET_VAR(_)              => 0x6C,
            Opcode::DEREF_IMM(_, _)         => 0x6D,
            Opcode::DEREF_VAR(_, _)         => 0x6E,
            Opcode::REF_IMM(_, _)           => 0x6F,
            Opcode::REF_VAR(_, _)           => 0x70,
            Opcode::INST_NAME(_)            => 0x71,
            Opcode::INST_VAR(_)             => 0x72,
            Opcode::MOD_I_I(_, _, _)        => 0x73,
            Opcode::MOD_V_I(_, _, _)        => 0x74,
            Opcode::MOD_I_V(_, _, _)        => 0x75,
            Opcode::MOD_V_V(_, _, _)        => 0x76,
            Opcode::PMOV_IMM_IMM(_, _, _)   => 0x77,
            Opcode::PMOV_VAR_IMM(_, _, _)   => 0x78,
            Opcode::PMOV_IMM_VAR(_, _, _)   => 0x79,
            Opcode::PMOV_VAR_VAR(_, _, _)   => 0x7A,
            Opcode::ALLOC_TYPE_IMM(_, _, _) => 0x7B,
            Opcode::ALLOC_VAR_IMM(_, _, _)  => 0x7C,
            Opcode::ALLOC_TYPE_VAR(_, _, _) => 0x7D,
            Opcode::ALLOC_VAR_VAR(_, _, _)  => 0x7E,
            Opcode::FREE_VAR(_)             => 0x7F,
            Opcode::FREE_IMM_IMM(_, _)      => 0x80,
            Opcode::FREE_VAR_IMM(_, _)      => 0x81,
            Opcode::FREE_IMM_VAR(_, _)      => 0x82,
            Opcode::FREE_VAR_VAR(_, _)      => 0x83,
        }
    }
}

#[derive(Debug, Clone)]
pub struct Instruction {
    pub index: usize,

    pub opcode: Opcode,
}