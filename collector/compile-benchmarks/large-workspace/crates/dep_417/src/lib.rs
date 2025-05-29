pub fn code() {
    println!("Hello from dep_417");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_417");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_417: {t}");
}

pub fn foo() {
    dep_296::code();
    dep_296::code_inlined();
    dep_296::code_generic(1u32);
    dep_269::code();
    dep_269::code_inlined();
    dep_269::code_generic(1u32);
    dep_172::code();
    dep_172::code_inlined();
    dep_172::code_generic(1u32);
    dep_388::code();
    dep_388::code_inlined();
    dep_388::code_generic(1u32);
    dep_160::code();
    dep_160::code_inlined();
    dep_160::code_generic(1u32);
}
