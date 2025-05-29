pub fn code() {
    println!("Hello from dep_587");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_587");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_587: {t}");
}

pub fn foo() {
    dep_248::code();
    dep_248::code_inlined();
    dep_248::code_generic(1u32);
    dep_331::code();
    dep_331::code_inlined();
    dep_331::code_generic(1u32);
    dep_379::code();
    dep_379::code_inlined();
    dep_379::code_generic(1u32);
    dep_292::code();
    dep_292::code_inlined();
    dep_292::code_generic(1u32);
    dep_351::code();
    dep_351::code_inlined();
    dep_351::code_generic(1u32);
}
