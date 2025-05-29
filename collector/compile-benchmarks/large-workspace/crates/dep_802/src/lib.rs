pub fn code() {
    println!("Hello from dep_802");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_802");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_802: {t}");
}

pub fn foo() {
    dep_390::code();
    dep_390::code_inlined();
    dep_390::code_generic(1u32);
}
