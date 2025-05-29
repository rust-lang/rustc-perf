pub fn code() {
    println!("Hello from dep_27");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_27");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_27: {t}");
}

pub fn foo() {
    dep_0::code();
    dep_0::code_inlined();
    dep_0::code_generic(1u32);
    dep_9::code();
    dep_9::code_inlined();
    dep_9::code_generic(1u32);
}
