pub fn code() {
    println!("Hello from dep_666");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_666");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_666: {t}");
}

pub fn foo() {
    dep_239::code();
    dep_239::code_inlined();
    dep_239::code_generic(1u32);
    dep_352::code();
    dep_352::code_inlined();
    dep_352::code_generic(1u32);
    dep_221::code();
    dep_221::code_inlined();
    dep_221::code_generic(1u32);
}
