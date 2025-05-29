pub fn code() {
    println!("Hello from dep_493");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_493");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_493: {t}");
}

pub fn foo() {
    dep_244::code();
    dep_244::code_inlined();
    dep_244::code_generic(1u32);
}
