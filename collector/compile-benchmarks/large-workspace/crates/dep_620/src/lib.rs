pub fn code() {
    println!("Hello from dep_620");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_620");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_620: {t}");
}

pub fn foo() {
    dep_160::code();
    dep_160::code_inlined();
    dep_160::code_generic(1u32);
    dep_216::code();
    dep_216::code_inlined();
    dep_216::code_generic(1u32);
}
