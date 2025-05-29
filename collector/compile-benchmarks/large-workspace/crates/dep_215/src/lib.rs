pub fn code() {
    println!("Hello from dep_215");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_215");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_215: {t}");
}

pub fn foo() {
    dep_89::code();
    dep_89::code_inlined();
    dep_89::code_generic(1u32);
    dep_130::code();
    dep_130::code_inlined();
    dep_130::code_generic(1u32);
    dep_101::code();
    dep_101::code_inlined();
    dep_101::code_generic(1u32);
    dep_63::code();
    dep_63::code_inlined();
    dep_63::code_generic(1u32);
    dep_145::code();
    dep_145::code_inlined();
    dep_145::code_generic(1u32);
}
