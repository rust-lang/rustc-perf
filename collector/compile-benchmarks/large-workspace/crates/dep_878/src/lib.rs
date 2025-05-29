pub fn code() {
    println!("Hello from dep_878");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_878");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_878: {t}");
}

pub fn foo() {
    dep_161::code();
    dep_161::code_inlined();
    dep_161::code_generic(1u32);
    dep_232::code();
    dep_232::code_inlined();
    dep_232::code_generic(1u32);
    dep_241::code();
    dep_241::code_inlined();
    dep_241::code_generic(1u32);
}
