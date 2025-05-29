pub fn code() {
    println!("Hello from dep_684");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_684");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_684: {t}");
}

pub fn foo() {
    dep_237::code();
    dep_237::code_inlined();
    dep_237::code_generic(1u32);
    dep_224::code();
    dep_224::code_inlined();
    dep_224::code_generic(1u32);
}
