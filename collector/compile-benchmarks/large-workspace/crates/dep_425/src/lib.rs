pub fn code() {
    println!("Hello from dep_425");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_425");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_425: {t}");
}

pub fn foo() {
    dep_289::code();
    dep_289::code_inlined();
    dep_289::code_generic(1u32);
    dep_223::code();
    dep_223::code_inlined();
    dep_223::code_generic(1u32);
    dep_203::code();
    dep_203::code_inlined();
    dep_203::code_generic(1u32);
    dep_205::code();
    dep_205::code_inlined();
    dep_205::code_generic(1u32);
    dep_333::code();
    dep_333::code_inlined();
    dep_333::code_generic(1u32);
}
