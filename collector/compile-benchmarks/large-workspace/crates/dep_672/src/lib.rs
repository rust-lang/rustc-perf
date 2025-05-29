pub fn code() {
    println!("Hello from dep_672");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_672");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_672: {t}");
}

pub fn foo() {
    dep_225::code();
    dep_225::code_inlined();
    dep_225::code_generic(1u32);
    dep_296::code();
    dep_296::code_inlined();
    dep_296::code_generic(1u32);
}
