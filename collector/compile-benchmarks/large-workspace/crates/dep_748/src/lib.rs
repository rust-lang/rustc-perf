pub fn code() {
    println!("Hello from dep_748");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_748");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_748: {t}");
}

pub fn foo() {
    dep_408::code();
    dep_408::code_inlined();
    dep_408::code_generic(1u32);
    dep_240::code();
    dep_240::code_inlined();
    dep_240::code_generic(1u32);
    dep_359::code();
    dep_359::code_inlined();
    dep_359::code_generic(1u32);
    dep_372::code();
    dep_372::code_inlined();
    dep_372::code_generic(1u32);
}
