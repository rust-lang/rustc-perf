pub fn code() {
    println!("Hello from dep_665");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_665");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_665: {t}");
}

pub fn foo() {
    dep_204::code();
    dep_204::code_inlined();
    dep_204::code_generic(1u32);
    dep_315::code();
    dep_315::code_inlined();
    dep_315::code_generic(1u32);
    dep_253::code();
    dep_253::code_inlined();
    dep_253::code_generic(1u32);
    dep_347::code();
    dep_347::code_inlined();
    dep_347::code_generic(1u32);
}
