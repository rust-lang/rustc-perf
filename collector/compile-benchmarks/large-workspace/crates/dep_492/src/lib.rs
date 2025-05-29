pub fn code() {
    println!("Hello from dep_492");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_492");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_492: {t}");
}

pub fn foo() {
    dep_163::code();
    dep_163::code_inlined();
    dep_163::code_generic(1u32);
    dep_235::code();
    dep_235::code_inlined();
    dep_235::code_generic(1u32);
    dep_213::code();
    dep_213::code_inlined();
    dep_213::code_generic(1u32);
    dep_172::code();
    dep_172::code_inlined();
    dep_172::code_generic(1u32);
}
