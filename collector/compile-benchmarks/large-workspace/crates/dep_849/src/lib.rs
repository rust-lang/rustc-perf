pub fn code() {
    println!("Hello from dep_849");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_849");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_849: {t}");
}

pub fn foo() {
    dep_357::code();
    dep_357::code_inlined();
    dep_357::code_generic(1u32);
    dep_221::code();
    dep_221::code_inlined();
    dep_221::code_generic(1u32);
}
