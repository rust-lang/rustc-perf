pub fn code() {
    println!("Hello from dep_18");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_18");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_18: {t}");
}

pub fn foo() {
    dep_9::code();
    dep_9::code_inlined();
    dep_9::code_generic(1u32);
    dep_2::code();
    dep_2::code_inlined();
    dep_2::code_generic(1u32);
    dep_4::code();
    dep_4::code_inlined();
    dep_4::code_generic(1u32);
}
