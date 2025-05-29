pub fn code() {
    println!("Hello from dep_894");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_894");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_894: {t}");
}

pub fn foo() {
    dep_276::code();
    dep_276::code_inlined();
    dep_276::code_generic(1u32);
    dep_402::code();
    dep_402::code_inlined();
    dep_402::code_generic(1u32);
    dep_360::code();
    dep_360::code_inlined();
    dep_360::code_generic(1u32);
    dep_284::code();
    dep_284::code_inlined();
    dep_284::code_generic(1u32);
}
