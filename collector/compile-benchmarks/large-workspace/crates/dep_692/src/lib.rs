pub fn code() {
    println!("Hello from dep_692");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_692");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_692: {t}");
}

pub fn foo() {
    dep_258::code();
    dep_258::code_inlined();
    dep_258::code_generic(1u32);
    dep_354::code();
    dep_354::code_inlined();
    dep_354::code_generic(1u32);
    dep_160::code();
    dep_160::code_inlined();
    dep_160::code_generic(1u32);
    dep_169::code();
    dep_169::code_inlined();
    dep_169::code_generic(1u32);
    dep_244::code();
    dep_244::code_inlined();
    dep_244::code_generic(1u32);
}
