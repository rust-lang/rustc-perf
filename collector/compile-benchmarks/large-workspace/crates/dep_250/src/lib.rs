pub fn code() {
    println!("Hello from dep_250");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_250");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_250: {t}");
}

pub fn foo() {
    dep_61::code();
    dep_61::code_inlined();
    dep_61::code_generic(1u32);
    dep_120::code();
    dep_120::code_inlined();
    dep_120::code_generic(1u32);
    dep_92::code();
    dep_92::code_inlined();
    dep_92::code_generic(1u32);
    dep_108::code();
    dep_108::code_inlined();
    dep_108::code_generic(1u32);
}
