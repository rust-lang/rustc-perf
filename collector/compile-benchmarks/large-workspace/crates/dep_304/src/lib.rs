pub fn code() {
    println!("Hello from dep_304");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_304");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_304: {t}");
}

pub fn foo() {
    dep_153::code();
    dep_153::code_inlined();
    dep_153::code_generic(1u32);
    dep_141::code();
    dep_141::code_inlined();
    dep_141::code_generic(1u32);
    dep_77::code();
    dep_77::code_inlined();
    dep_77::code_generic(1u32);
    dep_157::code();
    dep_157::code_inlined();
    dep_157::code_generic(1u32);
    dep_142::code();
    dep_142::code_inlined();
    dep_142::code_generic(1u32);
}
