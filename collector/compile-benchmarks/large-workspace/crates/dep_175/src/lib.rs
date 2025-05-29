pub fn code() {
    println!("Hello from dep_175");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_175");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_175: {t}");
}

pub fn foo() {
    dep_104::code();
    dep_104::code_inlined();
    dep_104::code_generic(1u32);
    dep_84::code();
    dep_84::code_inlined();
    dep_84::code_generic(1u32);
    dep_123::code();
    dep_123::code_inlined();
    dep_123::code_generic(1u32);
    dep_82::code();
    dep_82::code_inlined();
    dep_82::code_generic(1u32);
    dep_116::code();
    dep_116::code_inlined();
    dep_116::code_generic(1u32);
}
