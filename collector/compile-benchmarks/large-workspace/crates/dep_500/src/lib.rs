pub fn code() {
    println!("Hello from dep_500");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_500");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_500: {t}");
}

pub fn foo() {
    dep_241::code();
    dep_241::code_inlined();
    dep_241::code_generic(1u32);
}
