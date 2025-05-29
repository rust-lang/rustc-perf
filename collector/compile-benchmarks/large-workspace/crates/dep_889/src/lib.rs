pub fn code() {
    println!("Hello from dep_889");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_889");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_889: {t}");
}

pub fn foo() {
    dep_363::code();
    dep_363::code_inlined();
    dep_363::code_generic(1u32);
    dep_220::code();
    dep_220::code_inlined();
    dep_220::code_generic(1u32);
}
