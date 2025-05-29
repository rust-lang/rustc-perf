pub fn code() {
    println!("Hello from dep_489");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_489");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_489: {t}");
}

pub fn foo() {
    dep_254::code();
    dep_254::code_inlined();
    dep_254::code_generic(1u32);
    dep_185::code();
    dep_185::code_inlined();
    dep_185::code_generic(1u32);
    dep_371::code();
    dep_371::code_inlined();
    dep_371::code_generic(1u32);
    dep_170::code();
    dep_170::code_inlined();
    dep_170::code_generic(1u32);
    dep_308::code();
    dep_308::code_inlined();
    dep_308::code_generic(1u32);
}
