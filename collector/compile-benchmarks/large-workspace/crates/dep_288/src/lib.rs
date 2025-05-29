pub fn code() {
    println!("Hello from dep_288");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_288");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_288: {t}");
}

pub fn foo() {
    dep_78::code();
    dep_78::code_inlined();
    dep_78::code_generic(1u32);
}
