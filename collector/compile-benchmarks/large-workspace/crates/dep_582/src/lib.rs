pub fn code() {
    println!("Hello from dep_582");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_582");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_582: {t}");
}

pub fn foo() {
    dep_331::code();
    dep_331::code_inlined();
    dep_331::code_generic(1u32);
    dep_379::code();
    dep_379::code_inlined();
    dep_379::code_generic(1u32);
    dep_338::code();
    dep_338::code_inlined();
    dep_338::code_generic(1u32);
    dep_336::code();
    dep_336::code_inlined();
    dep_336::code_generic(1u32);
}
