pub fn code() {
    println!("Hello from dep_715");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_715");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_715: {t}");
}

pub fn foo() {
    dep_294::code();
    dep_294::code_inlined();
    dep_294::code_generic(1u32);
    dep_406::code();
    dep_406::code_inlined();
    dep_406::code_generic(1u32);
}
