pub fn code() {
    println!("Hello from dep_355");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_355");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_355: {t}");
}

pub fn foo() {
    dep_149::code();
    dep_149::code_inlined();
    dep_149::code_generic(1u32);
    dep_137::code();
    dep_137::code_inlined();
    dep_137::code_generic(1u32);
    dep_107::code();
    dep_107::code_inlined();
    dep_107::code_generic(1u32);
    dep_96::code();
    dep_96::code_inlined();
    dep_96::code_generic(1u32);
}
