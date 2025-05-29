pub fn code() {
    println!("Hello from dep_455");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_455");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_455: {t}");
}

pub fn foo() {
    dep_284::code();
    dep_284::code_inlined();
    dep_284::code_generic(1u32);
    dep_164::code();
    dep_164::code_inlined();
    dep_164::code_generic(1u32);
    dep_395::code();
    dep_395::code_inlined();
    dep_395::code_generic(1u32);
    dep_163::code();
    dep_163::code_inlined();
    dep_163::code_generic(1u32);
}
