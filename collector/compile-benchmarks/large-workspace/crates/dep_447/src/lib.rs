pub fn code() {
    println!("Hello from dep_447");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_447");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_447: {t}");
}

pub fn foo() {
    dep_374::code();
    dep_374::code_inlined();
    dep_374::code_generic(1u32);
    dep_206::code();
    dep_206::code_inlined();
    dep_206::code_generic(1u32);
    dep_256::code();
    dep_256::code_inlined();
    dep_256::code_generic(1u32);
    dep_268::code();
    dep_268::code_inlined();
    dep_268::code_generic(1u32);
}
