use super::*;
use delimited::Delimited;

ast_struct! {
    /// An expression.
    pub struct Expr {
        /// Type of the expression.
        pub node: ExprKind,

        /// Attributes tagged on the expression.
        pub attrs: Vec<u64>,
    }
}

ast_enum_of_structs! {
    pub enum ExprKind {
        /// A `box x` expression.
        pub Box(ExprBox {
            pub expr: Box<u64>,
            pub box_token: tokens::Box_,
        }),

        /// E.g. 'place <- val' or `in place { val }`.
        pub InPlace(ExprInPlace {
            pub place: Box<u64>,
            pub kind: InPlaceKind,
            pub value: Box<u64>,
        }),

        /// An array, e.g. `[a, b, c, d]`.
        pub Array(ExprArray {
            pub exprs: Delimited,
            pub bracket_token: tokens::Bracket,
        }),

        /// A function call.
        pub Call(ExprCall {
            pub func: Box<u64>,
            pub args: Delimited,
            pub paren_token: tokens::Paren,
        }),

        /// A method call (`x.foo::<Bar, Baz>(a, b, c, d)`)
        ///
        /// The `Ident` is the identifier for the method name.
        /// The vector of `Ty`s are the ascripted type parameters for the method
        /// (within the angle brackets).
        ///
        /// Thus, `x.foo::<Bar, Baz>(a, b, c, d)` is represented as
        /// `ExprKind::MethodCall(foo, [Bar, Baz], [x, a, b, c, d])`.
        pub MethodCall(ExprMethodCall {
            pub expr: Box<u64>,
            pub method: Ident,
            pub typarams: Delimited,
            pub args: Delimited,
            pub paren_token: tokens::Paren,
            pub dot_token: tokens::Dot,
            pub lt_token: Option<tokens::Lt>,
            pub colon2_token: Option<tokens::Colon2>,
            pub gt_token: Option<tokens::Gt>,
        }),

        /// A tuple, e.g. `(a, b, c, d)`.
        pub Tup(ExprTup {
            pub args: Delimited,
            pub paren_token: tokens::Paren,
            pub lone_comma: Option<tokens::Comma>,
        }),

        /// A binary operation, e.g. `a + b`, `a * b`.
        pub Binary(ExprBinary {
            pub op: BinOp,
            pub left: Box<u64>,
            pub right: Box<u64>,
        }),

        /// A unary operation, e.g. `!x`, `*x`.
        pub Unary(ExprUnary {
            pub op: UnOp,
            pub expr: Box<u64>,
        }),

        /// A literal, e.g. `1`, `"foo"`.
        pub Lit(Lit),

        /// A cast, e.g. `foo as f64`.
        pub Cast(ExprCast {
            pub expr: Box<u64>,
            pub as_token: tokens::As,
            pub ty: Box<u64>,
        }),

        /// A type ascription, e.g. `foo: f64`.
        pub Type(ExprType {
            pub expr: Box<u64>,
            pub colon_token: tokens::Colon,
            pub ty: Box<u64>,
        }),

        /// An `if` block, with an optional else block
        ///
        /// E.g., `if expr { block } else { expr }`
        pub If(ExprIf {
            pub cond: Box<u64>,
            pub if_true: Block,
            pub if_false: Option<Box<u64>>,
            pub if_token: tokens::If,
            pub else_token: Option<tokens::Else>,
        }),

        /// An `if let` expression with an optional else block
        ///
        /// E.g., `if let pat = expr { block } else { expr }`
        ///
        /// This is desugared to a `match` expression.
        pub IfLet(ExprIfLet {
            pub pat: Box<u64>,
            pub expr: Box<u64>,
            pub if_true: Block,
            pub if_false: Option<Box<u64>>,
            pub if_token: tokens::If,
            pub let_token: tokens::Let,
            pub eq_token: tokens::Eq,
            pub else_token: Option<tokens::Else>,
        }),

        /// A while loop, with an optional label
        ///
        /// E.g., `'label: while expr { block }`
        pub While(ExprWhile {
            pub cond: Box<u64>,
            pub body: Block,
            pub label: Option<Lifetime>,
            pub colon_token: Option<tokens::Colon>,
            pub while_token: tokens::While,
        }),

        /// A while-let loop, with an optional label.
        ///
        /// E.g., `'label: while let pat = expr { block }`
        ///
        /// This is desugared to a combination of `loop` and `match` expressions.
        pub WhileLet(ExprWhileLet {
            pub pat: Box<u64>,
            pub expr: Box<u64>,
            pub body: Block,
            pub label: Option<Lifetime>,
            pub colon_token: Option<tokens::Colon>,
            pub while_token: tokens::While,
            pub let_token: tokens::Let,
            pub eq_token: tokens::Eq,
        }),

        /// A for loop, with an optional label.
        ///
        /// E.g., `'label: for pat in expr { block }`
        ///
        /// This is desugared to a combination of `loop` and `match` expressions.
        pub ForLoop(ExprForLoop {
            pub pat: Box<u64>,
            pub expr: Box<u64>,
            pub body: Block,
            pub label: Option<Lifetime>,
            pub for_token: tokens::For,
            pub colon_token: Option<tokens::Colon>,
            pub in_token: tokens::In,
        }),

        /// Conditionless loop with an optional label.
        ///
        /// E.g. `'label: loop { block }`
        pub Loop(ExprLoop {
            pub body: Block,
            pub label: Option<Lifetime>,
            pub loop_token: tokens::Loop,
            pub colon_token: Option<tokens::Colon>,
        }),

        /// A `match` block.
        pub Match(ExprMatch {
            pub match_token: tokens::Match,
            pub brace_token: tokens::Brace,
            pub expr: Box<u64>,
            pub arms: Vec<u64>,
        }),

        /// A closure (for example, `move |a, b, c| a + b + c`)
        pub Closure(ExprClosure {
            pub capture: CaptureBy,
            pub decl: Box<u64>,
            pub body: Box<u64>,
            pub or1_token: tokens::Or,
            pub or2_token: tokens::Or,
        }),

        /// A block (`{ ... }` or `unsafe { ... }`)
        pub Block(ExprBlock {
            pub unsafety: Unsafety,
            pub block: Block,
        }),

        /// An assignment (`a = foo()`)
        pub Assign(ExprAssign {
            pub left: Box<u64>,
            pub right: Box<u64>,
            pub eq_token: tokens::Eq,
        }),

        /// An assignment with an operator
        ///
        /// For example, `a += 1`.
        pub AssignOp(ExprAssignOp {
            pub op: BinOp,
            pub left: Box<u64>,
            pub right: Box<u64>,
        }),

        /// Access of a named struct field (`obj.foo`)
        pub Field(ExprField {
            pub expr: Box<u64>,
            pub field: Ident,
            pub dot_token: tokens::Dot,
        }),

        /// Access of an unnamed field of a struct or tuple-struct
        ///
        /// For example, `foo.0`.
        pub TupField(ExprTupField {
            pub expr: Box<u64>,
            pub field: Lit,
            pub dot_token: tokens::Dot,
        }),

        /// An indexing operation (`foo[2]`)
        pub Index(ExprIndex {
            pub expr: Box<u64>,
            pub index: Box<u64>,
            pub bracket_token: tokens::Bracket,
        }),

        /// A range (`1..2`, `1..`, `..2`, `1...2`, `1...`, `...2`)
        pub Range(ExprRange {
            pub from: Option<Box<u64>>,
            pub to: Option<Box<u64>>,
            pub limits: RangeLimits,
        }),

        /// Variable reference, possibly containing `::` and/or type
        /// parameters, e.g. foo::bar::<baz>.
        ///
        /// Optionally "qualified",
        /// E.g. `<Vec<T> as SomeTrait>::SomeType`.
        pub Path(ExprPath {
            pub qself: Option<QSelf>,
            pub path: Path,
        }),

        /// A referencing operation (`&a` or `&mut a`)
        pub AddrOf(ExprAddrOf {
            pub and_token: tokens::And,
            pub mutbl: Mutability,
            pub expr: Box<u64>,
        }),

        /// A `break`, with an optional label to break, and an optional expression
        pub Break(ExprBreak {
            pub label: Option<Lifetime>,
            pub expr: Option<Box<u64>>,
            pub break_token: tokens::Break,
        }),

        /// A `continue`, with an optional label
        pub Continue(ExprContinue {
            pub label: Option<Lifetime>,
            pub continue_token: tokens::Continue,
        }),

        /// A `return`, with an optional value to be returned
        pub Ret(ExprRet {
            pub expr: Option<Box<u64>>,
            pub return_token: tokens::Return,
        }),

        /// A macro invocation; pre-expansion
        pub Mac(Mac),

        /// A struct literal expression.
        ///
        /// For example, `Foo {x: 1, y: 2}`, or
        /// `Foo {x: 1, .. base}`, where `base` is the `Option<Expr>`.
        pub Struct(ExprStruct {
            pub path: Path,
            pub fields: Delimited,
            pub rest: Option<Box<u64>>,
            pub dot2_token: Option<tokens::Dot2>,
            pub brace_token: tokens::Brace,
        }),

        /// An array literal constructed from one repeated element.
        ///
        /// For example, `[1; 5]`. The first expression is the element
        /// to be repeated; the second is the number of times to repeat it.
        pub Repeat(ExprRepeat {
            pub bracket_token: tokens::Bracket,
            pub semi_token: tokens::Semi,
            pub expr: Box<u64>,
            pub amt: Box<u64>,
        }),

        /// No-op: used solely so we can pretty-print faithfully
        pub Paren(ExprParen {
            pub expr: Box<u64>,
            pub paren_token: tokens::Paren,
        }),

        /// No-op: used solely so we can pretty-print faithfully
        ///
        /// A `group` represents a `None`-delimited span in the input
        /// `TokenStream` which affects the precidence of the resulting
        /// expression. They are used for macro hygiene.
        pub Group(ExprGroup {
            pub expr: Box<u64>,
            pub group_token: tokens::Group,
        }),

        /// `expr?`
        pub Try(ExprTry {
            pub expr: Box<u64>,
            pub question_token: tokens::Question,
        }),

        /// A catch expression.
        ///
        /// E.g. `do catch { block }`
        pub Catch(ExprCatch {
            pub do_token: tokens::Do,
            pub catch_token: tokens::Catch,
            pub block: Block,
        }),

        /// A yield expression.
        ///
        /// E.g. `yield expr`
        pub Yield(ExprYield {
            pub yield_token: tokens::Yield,
            pub expr: Option<Box<u64>>,
        }),
    }
}

ast_struct! {
    /// A Block (`{ .. }`).
    ///
    /// E.g. `{ .. }` as in `fn foo() { .. }`
    pub struct Block {
        pub brace_token: tokens::Brace,
        /// Statements in a block
        pub stmts: Vec<u64>,
    }
}

ast_enum! {
    /// A statement, usually ending in a semicolon.
    pub enum Stmt {
        /// A local (let) binding.
        Local(Box<u64>),

        /// An item definition.
        Item(Box<u64>),

        /// Expr without trailing semicolon.
        Expr(Box<u64>),

        /// Expression with trailing semicolon;
        Semi(Box<u64>, tokens::Semi),

        /// Macro invocation.
        Mac(Box<u64>),
    }
}

ast_enum! {
    /// How a macro was invoked.
    pub enum MacStmtStyle {
        /// The macro statement had a trailing semicolon, e.g. `foo! { ... };`
        /// `foo!(...);`, `foo![...];`
        Semicolon(tokens::Semi),

        /// The macro statement had braces; e.g. foo! { ... }
        Braces,

        /// The macro statement had parentheses or brackets and no semicolon; e.g.
        /// `foo!(...)`. All of these will end up being converted into macro
        /// expressions.
        NoBraces,
    }
}

ast_enum_of_structs! {
    // Clippy false positive
    // https://github.com/Manishearth/rust-clippy/issues/1241
    #[cfg_attr(feature = "cargo-clippy", allow(enum_variant_names))]
    pub enum Pat {
        /// Represents a wildcard pattern (`_`)
        pub Wild(PatWild {
            pub underscore_token: tokens::Underscore,
        }),

        /// A `Pat::Ident` may either be a new bound variable (`ref mut binding @ OPT_SUBPATTERN`),
        /// or a unit struct/variant pattern, or a const pattern (in the last two cases the third
        /// field must be `None`). Disambiguation cannot be done with parser alone, so it happens
        /// during name resolution.
        pub Ident(PatIdent {
            pub mode: BindingMode,
            pub ident: Ident,
            pub subpat: Option<Box<u64>>,
            pub at_token: Option<tokens::At>,
        }),

        /// A struct or struct variant pattern, e.g. `Variant {x, y, ..}`.
        /// The `bool` is `true` in the presence of a `..`.
        pub Struct(PatStruct {
            pub path: Path,
            pub fields: Delimited,
            pub brace_token: tokens::Brace,
            pub dot2_token: Option<tokens::Dot2>,
        }),

        /// A tuple struct/variant pattern `Variant(x, y, .., z)`.
        /// If the `..` pattern fragment is present, then `Option<usize>` denotes its position.
        /// 0 <= position <= subpats.len()
        pub TupleStruct(PatTupleStruct {
            pub path: Path,
            pub pat: PatTuple,
        }),

        /// A possibly qualified path pattern.
        /// Unquailfied path patterns `A::B::C` can legally refer to variants, structs, constants
        /// or associated constants. Quailfied path patterns `<A>::B::C`/`<A as Trait>::B::C` can
        /// only legally refer to associated constants.
        pub Path(PatPath {
            pub qself: Option<QSelf>,
            pub path: Path,
        }),

        /// A tuple pattern `(a, b)`.
        /// If the `..` pattern fragment is present, then `Option<usize>` denotes its position.
        /// 0 <= position <= subpats.len()
        pub Tuple(PatTuple {
            pub pats: Delimited,
            pub dots_pos: Option<usize>,
            pub paren_token: tokens::Paren,
            pub dot2_token: Option<tokens::Dot2>,
            pub comma_token: Option<tokens::Comma>,
        }),
        /// A `box` pattern
        pub Box(PatBox {
            pub pat: Box<u64>,
            pub box_token: tokens::Box_,
        }),
        /// A reference pattern, e.g. `&mut (a, b)`
        pub Ref(PatRef {
            pub pat: Box<u64>,
            pub mutbl: Mutability,
            pub and_token: tokens::And,
        }),
        /// A literal
        pub Lit(PatLit {
            pub expr: Box<u64>,
        }),
        /// A range pattern, e.g. `1...2`
        pub Range(PatRange {
            pub lo: Box<u64>,
            pub hi: Box<u64>,
            pub limits: RangeLimits,
        }),
        /// `[a, b, i.., y, z]` is represented as:
        pub Slice(PatSlice {
            pub front: Delimited,
            pub middle: Option<Box<u64>>,
            pub back: Delimited,
            pub dot2_token: Option<tokens::Dot2>,
            pub comma_token: Option<tokens::Comma>,
            pub bracket_token: tokens::Bracket,
        }),
        /// A macro pattern; pre-expansion
        pub Mac(Mac),
    }
}

ast_struct! {
    /// An arm of a 'match'.
    ///
    /// E.g. `0...10 => { println!("match!") }` as in
    ///
    /// ```rust,ignore
    /// match n {
    ///     0...10 => { println!("match!") },
    ///     // ..
    /// }
    /// ```
    pub struct Arm {
        pub attrs: Vec<u64>,
        pub pats: Delimited,
        pub if_token: Option<tokens::If>,
        pub guard: Option<Box<u64>>,
        pub rocket_token: tokens::Rocket,
        pub body: Box<u64>,
        pub comma: Option<tokens::Comma>,
    }
}

ast_enum! {
    /// A capture clause
    pub enum CaptureBy {
        Value(tokens::Move),
        Ref,
    }
}

ast_enum! {
    /// Limit types of a range (inclusive or exclusive)
    pub enum RangeLimits {
        /// Inclusive at the beginning, exclusive at the end
        HalfOpen(tokens::Dot2),
        /// Inclusive at the beginning and end
        Closed(tokens::Dot3),
    }
}

ast_struct! {
    /// A single field in a struct pattern
    ///
    /// Patterns like the fields of Foo `{ x, ref y, ref mut z }`
    /// are treated the same as `x: x, y: ref y, z: ref mut z`,
    /// except `is_shorthand` is true
    pub struct FieldPat {
        /// The identifier for the field
        pub ident: Ident,
        /// The pattern the field is destructured to
        pub pat: Box<u64>,
        pub is_shorthand: bool,
        pub colon_token: Option<tokens::Colon>,
        pub attrs: Vec<u64>,
    }
}

ast_enum! {
    pub enum BindingMode {
        ByRef(tokens::Ref, Mutability),
        ByValue(Mutability),
    }
}

ast_enum! {
    pub enum InPlaceKind {
        Arrow(tokens::LArrow),
        In(tokens::In),
    }
}
