pub fn code() {
    println!("Hello from dep_795");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_795");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_795: {t}");
}

pub fn foo() {
    dep_370::code();
    dep_370::code_inlined();
    dep_370::code_generic(1u32);
    dep_267::code();
    dep_267::code_inlined();
    dep_267::code_generic(1u32);
    dep_165::code();
    dep_165::code_inlined();
    dep_165::code_generic(1u32);
    dep_269::code();
    dep_269::code_inlined();
    dep_269::code_generic(1u32);
}
