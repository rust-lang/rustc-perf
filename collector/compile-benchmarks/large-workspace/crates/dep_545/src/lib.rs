pub fn code() {
    println!("Hello from dep_545");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_545");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_545: {t}");
}

pub fn foo() {
    dep_164::code();
    dep_164::code_inlined();
    dep_164::code_generic(1u32);
    dep_191::code();
    dep_191::code_inlined();
    dep_191::code_generic(1u32);
    dep_358::code();
    dep_358::code_inlined();
    dep_358::code_generic(1u32);
}
