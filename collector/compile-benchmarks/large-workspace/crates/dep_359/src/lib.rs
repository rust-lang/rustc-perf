pub fn code() {
    println!("Hello from dep_359");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_359");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_359: {t}");
}

pub fn foo() {
    dep_137::code();
    dep_137::code_inlined();
    dep_137::code_generic(1u32);
    dep_70::code();
    dep_70::code_inlined();
    dep_70::code_generic(1u32);
    dep_132::code();
    dep_132::code_inlined();
    dep_132::code_generic(1u32);
    dep_121::code();
    dep_121::code_inlined();
    dep_121::code_generic(1u32);
    dep_129::code();
    dep_129::code_inlined();
    dep_129::code_generic(1u32);
}
