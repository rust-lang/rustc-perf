pub fn code() {
    println!("Hello from dep_321");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_321");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_321: {t}");
}

pub fn foo() {
    dep_61::code();
    dep_61::code_inlined();
    dep_61::code_generic(1u32);
    dep_156::code();
    dep_156::code_inlined();
    dep_156::code_generic(1u32);
    dep_96::code();
    dep_96::code_inlined();
    dep_96::code_generic(1u32);
    dep_87::code();
    dep_87::code_inlined();
    dep_87::code_generic(1u32);
    dep_120::code();
    dep_120::code_inlined();
    dep_120::code_generic(1u32);
}
