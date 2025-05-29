pub fn code() {
    println!("Hello from dep_338");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_338");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_338: {t}");
}

pub fn foo() {
    dep_137::code();
    dep_137::code_inlined();
    dep_137::code_generic(1u32);
    dep_135::code();
    dep_135::code_inlined();
    dep_135::code_generic(1u32);
}
