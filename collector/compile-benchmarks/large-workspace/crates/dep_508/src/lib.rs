pub fn code() {
    println!("Hello from dep_508");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_508");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_508: {t}");
}

pub fn foo() {
    dep_202::code();
    dep_202::code_inlined();
    dep_202::code_generic(1u32);
    dep_268::code();
    dep_268::code_inlined();
    dep_268::code_generic(1u32);
    dep_345::code();
    dep_345::code_inlined();
    dep_345::code_generic(1u32);
    dep_193::code();
    dep_193::code_inlined();
    dep_193::code_generic(1u32);
    dep_333::code();
    dep_333::code_inlined();
    dep_333::code_generic(1u32);
}
