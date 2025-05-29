pub fn code() {
    println!("Hello from dep_277");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_277");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_277: {t}");
}

pub fn foo() {
    dep_140::code();
    dep_140::code_inlined();
    dep_140::code_generic(1u32);
    dep_157::code();
    dep_157::code_inlined();
    dep_157::code_generic(1u32);
    dep_120::code();
    dep_120::code_inlined();
    dep_120::code_generic(1u32);
}
