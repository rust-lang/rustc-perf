pub fn code() {
    println!("Hello from dep_659");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_659");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_659: {t}");
}

pub fn foo() {
    dep_199::code();
    dep_199::code_inlined();
    dep_199::code_generic(1u32);
    dep_206::code();
    dep_206::code_inlined();
    dep_206::code_generic(1u32);
    dep_242::code();
    dep_242::code_inlined();
    dep_242::code_generic(1u32);
    dep_196::code();
    dep_196::code_inlined();
    dep_196::code_generic(1u32);
    dep_339::code();
    dep_339::code_inlined();
    dep_339::code_generic(1u32);
}
