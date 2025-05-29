pub fn code() {
    println!("Hello from dep_189");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_189");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_189: {t}");
}

pub fn foo() {
    dep_72::code();
    dep_72::code_inlined();
    dep_72::code_generic(1u32);
    dep_145::code();
    dep_145::code_inlined();
    dep_145::code_generic(1u32);
    dep_149::code();
    dep_149::code_inlined();
    dep_149::code_generic(1u32);
}
