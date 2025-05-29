pub fn code() {
    println!("Hello from dep_536");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_536");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_536: {t}");
}

pub fn foo() {
    dep_251::code();
    dep_251::code_inlined();
    dep_251::code_generic(1u32);
    dep_189::code();
    dep_189::code_inlined();
    dep_189::code_generic(1u32);
    dep_290::code();
    dep_290::code_inlined();
    dep_290::code_generic(1u32);
    dep_388::code();
    dep_388::code_inlined();
    dep_388::code_generic(1u32);
    dep_387::code();
    dep_387::code_inlined();
    dep_387::code_generic(1u32);
}
