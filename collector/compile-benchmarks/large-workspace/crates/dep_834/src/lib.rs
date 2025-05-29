pub fn code() {
    println!("Hello from dep_834");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_834");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_834: {t}");
}

pub fn foo() {
    dep_240::code();
    dep_240::code_inlined();
    dep_240::code_generic(1u32);
    dep_351::code();
    dep_351::code_inlined();
    dep_351::code_generic(1u32);
    dep_267::code();
    dep_267::code_inlined();
    dep_267::code_generic(1u32);
    dep_242::code();
    dep_242::code_inlined();
    dep_242::code_generic(1u32);
    dep_191::code();
    dep_191::code_inlined();
    dep_191::code_generic(1u32);
}
