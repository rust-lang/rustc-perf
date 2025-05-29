pub fn code() {
    println!("Hello from dep_725");
}

#[inline(always)]
pub fn code_inlined() {
    println!("Hello from dep_725");
}

pub fn code_generic<T>(t: T) where T: std::fmt::Display {
    println!("Hello from dep_725: {t}");
}

pub fn foo() {
    dep_241::code();
    dep_241::code_inlined();
    dep_241::code_generic(1u32);
    dep_404::code();
    dep_404::code_inlined();
    dep_404::code_generic(1u32);
    dep_361::code();
    dep_361::code_inlined();
    dep_361::code_generic(1u32);
}
