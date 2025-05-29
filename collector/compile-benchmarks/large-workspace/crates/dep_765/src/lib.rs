pub fn code() {
    println!("Hello from dep_765");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_765");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_765: {t}");
}

pub fn foo() {
    dep_374::code();
    dep_374::code_inlined();
    dep_374::code_generic(1u32);
    dep_232::code();
    dep_232::code_inlined();
    dep_232::code_generic(1u32);
    dep_248::code();
    dep_248::code_inlined();
    dep_248::code_generic(1u32);
    dep_336::code();
    dep_336::code_inlined();
    dep_336::code_generic(1u32);
}
