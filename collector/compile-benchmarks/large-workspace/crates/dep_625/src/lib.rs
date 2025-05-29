pub fn code() {
    println!("Hello from dep_625");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_625");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_625: {t}");
}

pub fn foo() {
    dep_280::code();
    dep_280::code_inlined();
    dep_280::code_generic(1u32);
    dep_277::code();
    dep_277::code_inlined();
    dep_277::code_generic(1u32);
    dep_330::code();
    dep_330::code_inlined();
    dep_330::code_generic(1u32);
    dep_299::code();
    dep_299::code_inlined();
    dep_299::code_generic(1u32);
}
