pub fn code() {
    println!("Hello from dep_54");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_54");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_54: {t}");
}

pub fn foo() {
    dep_2::code();
    dep_2::code_inlined();
    dep_2::code_generic(1u32);
    dep_9::code();
    dep_9::code_inlined();
    dep_9::code_generic(1u32);
    dep_1::code();
    dep_1::code_inlined();
    dep_1::code_generic(1u32);
}
