pub fn code() {
    println!("Hello from dep_675");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_675");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_675: {t}");
}

pub fn foo() {
    dep_317::code();
    dep_317::code_inlined();
    dep_317::code_generic(1u32);
    dep_238::code();
    dep_238::code_inlined();
    dep_238::code_generic(1u32);
    dep_213::code();
    dep_213::code_inlined();
    dep_213::code_generic(1u32);
}
