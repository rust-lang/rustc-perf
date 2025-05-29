pub fn code() {
    println!("Hello from dep_838");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_838");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_838: {t}");
}

pub fn foo() {
    dep_185::code();
    dep_185::code_inlined();
    dep_185::code_generic(1u32);
    dep_210::code();
    dep_210::code_inlined();
    dep_210::code_generic(1u32);
    dep_255::code();
    dep_255::code_inlined();
    dep_255::code_generic(1u32);
    dep_172::code();
    dep_172::code_inlined();
    dep_172::code_generic(1u32);
}
