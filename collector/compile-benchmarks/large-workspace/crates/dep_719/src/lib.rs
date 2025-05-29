pub fn code() {
    println!("Hello from dep_719");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_719");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_719: {t}");
}

pub fn foo() {
    dep_352::code();
    dep_352::code_inlined();
    dep_352::code_generic(1u32);
}
