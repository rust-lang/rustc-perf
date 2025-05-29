pub fn code() {
    println!("Hello from dep_677");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_677");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_677: {t}");
}

pub fn foo() {
    dep_382::code();
    dep_382::code_inlined();
    dep_382::code_generic(1u32);
    dep_254::code();
    dep_254::code_inlined();
    dep_254::code_generic(1u32);
    dep_160::code();
    dep_160::code_inlined();
    dep_160::code_generic(1u32);
    dep_192::code();
    dep_192::code_inlined();
    dep_192::code_generic(1u32);
}
