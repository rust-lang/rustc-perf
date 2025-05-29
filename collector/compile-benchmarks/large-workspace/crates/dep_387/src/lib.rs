pub fn code() {
    println!("Hello from dep_387");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_387");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_387: {t}");
}

pub fn foo() {
    dep_115::code();
    dep_115::code_inlined();
    dep_115::code_generic(1u32);
    dep_72::code();
    dep_72::code_inlined();
    dep_72::code_generic(1u32);
    dep_130::code();
    dep_130::code_inlined();
    dep_130::code_generic(1u32);
    dep_67::code();
    dep_67::code_inlined();
    dep_67::code_generic(1u32);
}
