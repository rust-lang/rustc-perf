pub fn code() {
    println!("Hello from dep_183");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_183");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_183: {t}");
}

pub fn foo() {
    dep_143::code();
    dep_143::code_inlined();
    dep_143::code_generic(1u32);
    dep_83::code();
    dep_83::code_inlined();
    dep_83::code_generic(1u32);
    dep_82::code();
    dep_82::code_inlined();
    dep_82::code_generic(1u32);
    dep_139::code();
    dep_139::code_inlined();
    dep_139::code_generic(1u32);
}
