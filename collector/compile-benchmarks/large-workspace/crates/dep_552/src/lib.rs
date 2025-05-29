pub fn code() {
    println!("Hello from dep_552");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_552");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_552: {t}");
}

pub fn foo() {
    dep_348::code();
    dep_348::code_inlined();
    dep_348::code_generic(1u32);
    dep_269::code();
    dep_269::code_inlined();
    dep_269::code_generic(1u32);
    dep_227::code();
    dep_227::code_inlined();
    dep_227::code_generic(1u32);
}
