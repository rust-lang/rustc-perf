pub fn code() {
    println!("Hello from dep_213");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_213");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_213: {t}");
}

pub fn foo() {
    dep_134::code();
    dep_134::code_inlined();
    dep_134::code_generic(1u32);
    dep_89::code();
    dep_89::code_inlined();
    dep_89::code_generic(1u32);
    dep_82::code();
    dep_82::code_inlined();
    dep_82::code_generic(1u32);
    dep_113::code();
    dep_113::code_inlined();
    dep_113::code_generic(1u32);
    dep_125::code();
    dep_125::code_inlined();
    dep_125::code_generic(1u32);
}
