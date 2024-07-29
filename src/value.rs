use crate::_type::Type;

#[derive(Debug, Clone)]
pub enum Values {
    VOID,
    // using basic wrappers to make type casting way easier
    SIGNED(i64),
    UNSIGNED(u64),
    DECIMAL(f64),
    POINTER(usize),
    STRUCT, // TODO
    TYPE(Type),
    NAME(String),
}

#[derive(Debug, Clone)]
pub struct Value {
    pub typ: Type,
    pub val: Values,
}

impl Value {
    pub fn set(&mut self, other: &Values) { // yeah it's just a wrapper what about it
        self.val.set(other);
    }
}

// TODO: actually make sure resulting numbers can fit in the types they're supposed to be
impl Values {
    pub fn set(&mut self, other: &Values) { // basically auto-type casting
        match(self, other) {
            (Values::VOID, _) => (),
            (Values::SIGNED(s), Values::VOID) => *s = 0,
            (Values::SIGNED(s), Values::SIGNED(v))       => *s = *v,
            (Values::SIGNED(s), Values::UNSIGNED(v))     => *s = *v as i64,
            (Values::SIGNED(s), Values::DECIMAL(v))      => *s = *v as i64,
            (Values::SIGNED(s), Values::POINTER(v))    => *s = *v as i64,
            (Values::SIGNED(_), Values::STRUCT)                          => panic!("cannot set a number value as a struct"),
            (Values::SIGNED(_), Values::TYPE(_))                         => panic!("cannot set a number value as a type"),
            (Values::SIGNED(_), Values::NAME(_))                         => panic!("cannot set a number value as a name"),
            (Values::UNSIGNED(s), Values::VOID)                => *s = 0,
            (Values::UNSIGNED(s), Values::SIGNED(v))     => *s = *v as u64,
            (Values::UNSIGNED(s), Values::UNSIGNED(v))   => *s = *v,
            (Values::UNSIGNED(s), Values::DECIMAL(v))    => *s = *v as u64,
            (Values::UNSIGNED(s), Values::POINTER(v))  => *s = *v as u64,
            (Values::UNSIGNED(_), Values::STRUCT)                        => panic!("cannot set a number value as a struct"),
            (Values::UNSIGNED(_), Values::TYPE(_))                       => panic!("cannot set a number value as a type"),
            (Values::UNSIGNED(_), Values::NAME(_))                       => panic!("cannot set a number value as a name"),
            (Values::DECIMAL(s), Values::VOID)                 => *s = 0.0,
            (Values::DECIMAL(s), Values::SIGNED(v))      => *s = *v as f64,
            (Values::DECIMAL(s), Values::UNSIGNED(v))    => *s = *v as f64,
            (Values::DECIMAL(s), Values::DECIMAL(v))     => *s = *v,
            (Values::DECIMAL(s), Values::POINTER(v))   => *s = *v as f64,
            (Values::DECIMAL(_), Values::STRUCT)                         => panic!("cannot set a number value as a struct"),
            (Values::DECIMAL(_), Values::TYPE(_))                        => panic!("cannot set a number value as a type"),
            (Values::DECIMAL(_), Values::NAME(_))                        => panic!("cannot set a number value as a name"),
            (Values::POINTER(s), Values::VOID)               => *s = 0,
            (Values::POINTER(s), Values::SIGNED(v))    => *s = *v as usize,
            (Values::POINTER(s), Values::UNSIGNED(v))  => *s = *v as usize,
            (Values::POINTER(s), Values::DECIMAL(v))   => *s = *v as usize,
            (Values::POINTER(s), Values::POINTER(v)) => *s = *v,
            (Values::POINTER(_), Values::STRUCT)                         => panic!("cannot set a pointer value as a struct"),
            (Values::POINTER(_), Values::TYPE(_))                        => panic!("cannot set a pointer value as a type"),
            (Values::POINTER(_), Values::NAME(_))                        => panic!("cannot set a pointer value as a name"),
            (Values::STRUCT, Values::VOID)                               => todo!(),
            (Values::STRUCT, Values::SIGNED(_))                          => todo!(),
            (Values::STRUCT, Values::UNSIGNED(_))                        => todo!(),
            (Values::STRUCT, Values::DECIMAL(_))                         => todo!(),
            (Values::STRUCT, Values::POINTER(_))                         => todo!(),
            (Values::STRUCT, Values::STRUCT)                             => todo!(),
            (Values::STRUCT, Values::TYPE(_))                            => todo!(),
            (Values::STRUCT, Values::NAME(_))                            => todo!(),
            (Values::TYPE(_), Values::VOID)                              => todo!(),
            (Values::TYPE(_), Values::SIGNED(_))                         => todo!(),
            (Values::TYPE(_), Values::UNSIGNED(_))                       => todo!(),
            (Values::TYPE(_), Values::DECIMAL(_))                        => todo!(),
            (Values::TYPE(_), Values::POINTER(_))                        => todo!(),
            (Values::TYPE(_), Values::STRUCT)                            => todo!(),
            (Values::TYPE(s), Values::TYPE(v))         => *s = v.clone(),
            (Values::TYPE(_), Values::NAME(_))                           => todo!(),
            (Values::NAME(s), Values::VOID)                 => *s = String::new(),
            (Values::NAME(_), Values::SIGNED(_))                         => panic!("cannot set a name value as a number"),
            (Values::NAME(_), Values::UNSIGNED(_))                       => panic!("cannot set a name value as a number"),
            (Values::NAME(_), Values::DECIMAL(_))                        => panic!("cannot set a name value as a number"),
            (Values::NAME(_), Values::POINTER(_))                        => panic!("cannot set a name value as a pointer"),
            (Values::NAME(_), Values::STRUCT)                            => panic!("cannot set a name value as a struct"),
            (Values::NAME(_), Values::TYPE(_))                           => panic!("cannot set a name value as a type"),
            (Values::NAME(s), Values::NAME(v))     => *s = v.clone(),
        }    
    }

    pub fn add(&self, other: &Values) -> Values {
        match(self, other) {
            (Values::VOID, _)                                        => Values::VOID,
            (Values::SIGNED(s), Values::VOID)                  => Values::SIGNED(*s),
            (Values::SIGNED(s), Values::SIGNED(v))       => Values::SIGNED(*s + *v),
            (Values::SIGNED(s), Values::UNSIGNED(v))     => Values::SIGNED(*s + *v as i64),
            (Values::SIGNED(s), Values::DECIMAL(v))      => Values::SIGNED(*s + *v as i64),
            (Values::SIGNED(s), Values::POINTER(v))    => Values::SIGNED(*s + *v as i64),
            (Values::SIGNED(_), Values::STRUCT)                      => panic!("cannot add struct to a number"),
            (Values::SIGNED(_), Values::TYPE(_))                     => panic!("cannot add type to a number"),
            (Values::SIGNED(_), Values::NAME(_))                     => panic!("cannot add name to a number"),
            (Values::UNSIGNED(s), Values::VOID)                => Values::UNSIGNED(*s),
            (Values::UNSIGNED(s), Values::SIGNED(v))     => Values::UNSIGNED(*s + *v as u64),
            (Values::UNSIGNED(s), Values::UNSIGNED(v))   => Values::UNSIGNED(*s + *v),
            (Values::UNSIGNED(s), Values::DECIMAL(v))    => Values::UNSIGNED(*s + *v as u64),
            (Values::UNSIGNED(s), Values::POINTER(v))  => Values::UNSIGNED(*s + *v as u64),
            (Values::UNSIGNED(_), Values::STRUCT)                    => panic!("cannot add struct to a number"),
            (Values::UNSIGNED(_), Values::TYPE(_))                   => panic!("cannot add type to a number"),
            (Values::UNSIGNED(_), Values::NAME(_))                   => panic!("cannot add name to a number"),
            (Values::DECIMAL(s), Values::VOID)                 => Values::DECIMAL(*s),
            (Values::DECIMAL(s), Values::SIGNED(v))      => Values::DECIMAL(*s + *v as f64),
            (Values::DECIMAL(s), Values::UNSIGNED(v))    => Values::DECIMAL(*s + *v as f64),
            (Values::DECIMAL(s), Values::DECIMAL(v))     => Values::DECIMAL(*s + *v),
            (Values::DECIMAL(s), Values::POINTER(v))   => Values::DECIMAL(*s + *v as f64),
            (Values::DECIMAL(_), Values::STRUCT)                     => panic!("cannot add struct to a number"),
            (Values::DECIMAL(_), Values::TYPE(_))                    => panic!("cannot add type to a number"),
            (Values::DECIMAL(_), Values::NAME(_))                    => panic!("cannot add name to a number"),
            (Values::POINTER(s), Values::VOID)               => Values::POINTER(*s),
            (Values::POINTER(s), Values::SIGNED(v))    => Values::POINTER(*s + *v as usize),
            (Values::POINTER(s), Values::UNSIGNED(v))  => Values::POINTER(*s + *v as usize),
            (Values::POINTER(s), Values::DECIMAL(v))   => Values::POINTER(*s + *v as usize),
            (Values::POINTER(s), Values::POINTER(v)) => Values::POINTER(*s + *v),
            (Values::POINTER(_), Values::STRUCT)                     => panic!("cannot add struct to a pointer"),
            (Values::POINTER(_), Values::TYPE(_))                    => panic!("cannot add type to a pointer"),
            (Values::POINTER(_), Values::NAME(_))                    => panic!("cannot add name to a pointer"),
            (Values::STRUCT, _)                                      => panic!("struct cannot be added to"),
            (Values::TYPE(_), _)                                     => panic!("type cannot be added to"),
            (Values::NAME(_), Values::VOID)                          => panic!("cannot add void to a name"),
            (Values::NAME(_), Values::SIGNED(_))                     => panic!("cannot add number to a name"),
            (Values::NAME(_), Values::UNSIGNED(_))                   => panic!("cannot add number to a name"),
            (Values::NAME(_), Values::DECIMAL(_))                    => panic!("cannot add number to a name"),
            (Values::NAME(_), Values::POINTER(_))                    => panic!("cannot add pointer to a name"),
            (Values::NAME(_), Values::STRUCT)                        => panic!("cannot add struct to a name"),
            (Values::NAME(_), Values::TYPE(_))                       => panic!("cannot add type to a name"),
            (Values::NAME(s), Values::NAME(v))     => Values::NAME(s.clone() + v),
        }
    }

    pub fn sub(&self, other: &Values) -> Values {
        match(self, other) {
            (Values::VOID, _) => Values::VOID,
            (Values::SIGNED(s), Values::VOID)                  => Values::SIGNED(*s),
            (Values::SIGNED(s), Values::SIGNED(v))       => Values::SIGNED(*s - *v),
            (Values::SIGNED(s), Values::UNSIGNED(v))     => Values::SIGNED(*s - *v as i64),
            (Values::SIGNED(s), Values::DECIMAL(v))      => Values::SIGNED(*s - *v as i64),
            (Values::SIGNED(s), Values::POINTER(v))    => Values::SIGNED(*s - *v as i64),
            (Values::SIGNED(_), Values::STRUCT)                      => panic!("cannot subtract struct from a number"),
            (Values::SIGNED(_), Values::TYPE(_))                     => panic!("cannot subtract type from a number"),
            (Values::SIGNED(_), Values::NAME(_))                     => panic!("cannot subtract name from a number"),
            (Values::UNSIGNED(s), Values::VOID)                => Values::UNSIGNED(*s),
            (Values::UNSIGNED(s), Values::SIGNED(v))     => Values::UNSIGNED(*s - *v as u64),
            (Values::UNSIGNED(s), Values::UNSIGNED(v))   => Values::UNSIGNED(*s - *v),
            (Values::UNSIGNED(s), Values::DECIMAL(v))    => Values::UNSIGNED(*s - *v as u64),
            (Values::UNSIGNED(s), Values::POINTER(v))  => Values::UNSIGNED(*s - *v as u64),
            (Values::UNSIGNED(_), Values::STRUCT)                    => panic!("cannot subtract struct from a number"),
            (Values::UNSIGNED(_), Values::TYPE(_))                   => panic!("cannot subtract type from a number"),
            (Values::UNSIGNED(_), Values::NAME(_))                   => panic!("cannot subtract name from a number"),
            (Values::DECIMAL(s), Values::VOID)                 => Values::DECIMAL(*s),
            (Values::DECIMAL(s), Values::SIGNED(v))      => Values::DECIMAL(*s - *v as f64),
            (Values::DECIMAL(s), Values::UNSIGNED(v))    => Values::DECIMAL(*s - *v as f64),
            (Values::DECIMAL(s), Values::DECIMAL(v))     => Values::DECIMAL(*s - *v),
            (Values::DECIMAL(s), Values::POINTER(v))   => Values::DECIMAL(*s - *v as f64),
            (Values::DECIMAL(_), Values::STRUCT)                     => panic!("cannot subtract struct from a number"),
            (Values::DECIMAL(_), Values::TYPE(_))                    => panic!("cannot subtract type from a number"),
            (Values::DECIMAL(_), Values::NAME(_))                    => panic!("cannot subtract name from a number"),
            (Values::POINTER(s), Values::VOID)               => Values::POINTER(*s),
            (Values::POINTER(s), Values::SIGNED(v))    => Values::POINTER(*s - *v as usize),
            (Values::POINTER(s), Values::UNSIGNED(v))  => Values::POINTER(*s - *v as usize),
            (Values::POINTER(s), Values::DECIMAL(v))   => Values::POINTER(*s - *v as usize),
            (Values::POINTER(s), Values::POINTER(v)) => Values::POINTER(*s - *v),
            (Values::POINTER(_), Values::STRUCT)                     => panic!("cannot subtract struct from a pointer"),
            (Values::POINTER(_), Values::TYPE(_))                    => panic!("cannot subtract type from a pointer"),
            (Values::POINTER(_), Values::NAME(_))                    => panic!("cannot subtract name from a pointer"),
            (Values::STRUCT, _)                                      => panic!("struct cannot be subtracted from"),
            (Values::TYPE(_), _)                                     => panic!("type cannot be subtracted from"),
            (Values::NAME(_), _)                                     => panic!("name cannot be subtracted from"),
        }
    }
    
    pub fn mul(&self, other: &Values) -> Values {
        match(self, other) {
            (Values::VOID, _) => Values::VOID,
            (Values::SIGNED(s), Values::VOID)                  => Values::SIGNED(*s),
            (Values::SIGNED(s), Values::SIGNED(v))       => Values::SIGNED(*s * *v),
            (Values::SIGNED(s), Values::UNSIGNED(v))     => Values::SIGNED(*s * *v as i64),
            (Values::SIGNED(s), Values::DECIMAL(v))      => Values::SIGNED(*s * *v as i64),
            (Values::SIGNED(s), Values::POINTER(v))    => Values::SIGNED(*s * *v as i64),
            (Values::SIGNED(_), Values::STRUCT)                      => panic!("cannot multiply number by a struct"),
            (Values::SIGNED(_), Values::TYPE(_))                     => panic!("cannot multiply number by a type"),
            (Values::SIGNED(_), Values::NAME(_))                     => panic!("cannot multiply number by a name"),
            (Values::UNSIGNED(s), Values::VOID)                => Values::UNSIGNED(*s),
            (Values::UNSIGNED(s), Values::SIGNED(v))     => Values::UNSIGNED(*s * *v as u64),
            (Values::UNSIGNED(s), Values::UNSIGNED(v))   => Values::UNSIGNED(*s * *v),
            (Values::UNSIGNED(s), Values::DECIMAL(v))    => Values::UNSIGNED(*s * *v as u64),
            (Values::UNSIGNED(s), Values::POINTER(v))  => Values::UNSIGNED(*s * *v as u64),
            (Values::UNSIGNED(_), Values::STRUCT)                    => panic!("cannot multiply number by a struct"),
            (Values::UNSIGNED(_), Values::TYPE(_))                   => panic!("cannot multiply number by a type"),
            (Values::UNSIGNED(_), Values::NAME(_))                   => panic!("cannot multiply number by a name"),
            (Values::DECIMAL(s), Values::VOID)                 => Values::DECIMAL(*s),
            (Values::DECIMAL(s), Values::SIGNED(v))      => Values::DECIMAL(*s * *v as f64),
            (Values::DECIMAL(s), Values::UNSIGNED(v))    => Values::DECIMAL(*s * *v as f64),
            (Values::DECIMAL(s), Values::DECIMAL(v))     => Values::DECIMAL(*s * *v),
            (Values::DECIMAL(s), Values::POINTER(v))   => Values::DECIMAL(*s * *v as f64),
            (Values::DECIMAL(_), Values::STRUCT)                     => panic!("cannot multiply number by a struct"),
            (Values::DECIMAL(_), Values::TYPE(_))                    => panic!("cannot multiply number by a type"),
            (Values::DECIMAL(_), Values::NAME(_))                    => panic!("cannot multiply number by a name"),
            (Values::POINTER(s), Values::VOID)               => Values::POINTER(*s),
            (Values::POINTER(s), Values::SIGNED(v))    => Values::POINTER(*s * *v as usize),
            (Values::POINTER(s), Values::UNSIGNED(v))  => Values::POINTER(*s * *v as usize),
            (Values::POINTER(s), Values::DECIMAL(v))   => Values::POINTER(*s * *v as usize),
            (Values::POINTER(s), Values::POINTER(v)) => Values::POINTER(*s * *v),
            (Values::POINTER(_), Values::STRUCT)                     => panic!("cannot multiply number by a struct"),
            (Values::POINTER(_), Values::TYPE(_))                    => panic!("cannot multiply number by a type"),
            (Values::POINTER(_), Values::NAME(_))                    => panic!("cannot multiply number by a name"),
            (Values::STRUCT, _)                                      => panic!("struct cannot be multiplied"),
            (Values::TYPE(_), _)                                     => panic!("type cannot be multiplied"),
            (Values::NAME(_), _)                                     => panic!("name cannot be multiplied"),
        }
    }
    
    pub fn div(&self, other: &Values) -> Values {
        match(self, other) {
            (Values::VOID, _) => Values::VOID,
            (Values::SIGNED(s), Values::VOID)                  => Values::SIGNED(*s),
            (Values::SIGNED(s), Values::SIGNED(v))       => Values::SIGNED(*s / *v),
            (Values::SIGNED(s), Values::UNSIGNED(v))     => Values::SIGNED(*s / *v as i64),
            (Values::SIGNED(s), Values::DECIMAL(v))      => Values::SIGNED(*s / *v as i64),
            (Values::SIGNED(s), Values::POINTER(v))    => Values::SIGNED(*s / *v as i64),
            (Values::SIGNED(_), Values::STRUCT)                      => panic!("cannot divide number by struct"),
            (Values::SIGNED(_), Values::TYPE(_))                     => panic!("cannot divide number by type"),
            (Values::SIGNED(_), Values::NAME(_))                     => panic!("cannot divide number by name"),
            (Values::UNSIGNED(s), Values::VOID)                => Values::UNSIGNED(*s),
            (Values::UNSIGNED(s), Values::SIGNED(v))     => Values::UNSIGNED(*s / *v as u64),
            (Values::UNSIGNED(s), Values::UNSIGNED(v))   => Values::UNSIGNED(*s / *v),
            (Values::UNSIGNED(s), Values::DECIMAL(v))    => Values::UNSIGNED(*s / *v as u64),
            (Values::UNSIGNED(s), Values::POINTER(v))  => Values::UNSIGNED(*s / *v as u64),
            (Values::UNSIGNED(_), Values::STRUCT)                    => panic!("cannot divide number by struct"),
            (Values::UNSIGNED(_), Values::TYPE(_))                   => panic!("cannot divide number by type"),
            (Values::UNSIGNED(_), Values::NAME(_))                   => panic!("cannot divide number by name"),
            (Values::DECIMAL(s), Values::VOID)                 => Values::DECIMAL(*s),
            (Values::DECIMAL(s), Values::SIGNED(v))      => Values::DECIMAL(*s / *v as f64),
            (Values::DECIMAL(s), Values::UNSIGNED(v))    => Values::DECIMAL(*s / *v as f64),
            (Values::DECIMAL(s), Values::DECIMAL(v))     => Values::DECIMAL(*s / *v),
            (Values::DECIMAL(s), Values::POINTER(v))   => Values::DECIMAL(*s / *v as f64),
            (Values::DECIMAL(_), Values::STRUCT)                     => panic!("cannot divide number by struct"),
            (Values::DECIMAL(_), Values::TYPE(_))                    => panic!("cannot divide number by type"),
            (Values::DECIMAL(_), Values::NAME(_))                    => panic!("cannot divide number by name"),
            (Values::POINTER(s), Values::VOID)               => Values::POINTER(*s),
            (Values::POINTER(s), Values::SIGNED(v))    => Values::POINTER(*s / *v as usize),
            (Values::POINTER(s), Values::UNSIGNED(v))  => Values::POINTER(*s / *v as usize),
            (Values::POINTER(s), Values::DECIMAL(v))   => Values::POINTER(*s / *v as usize),
            (Values::POINTER(s), Values::POINTER(v)) => Values::POINTER(*s / *v),
            (Values::POINTER(_), Values::STRUCT)                     => panic!("cannot divide struct by a pointer"),
            (Values::POINTER(_), Values::TYPE(_))                    => panic!("cannot divide type by a pointer"),
            (Values::POINTER(_), Values::NAME(_))                    => panic!("cannot divide name by a pointer"),
            (Values::STRUCT, _)                                      => panic!("struct cannot be divided"),
            (Values::TYPE(_), _)                                     => panic!("type cannot be divided"),
            (Values::NAME(_), _)                                     => panic!("name cannot be divided"),
        }
    }
}