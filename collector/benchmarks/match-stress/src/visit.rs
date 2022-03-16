use *;

pub fn visit_abi_kind<V>(_visitor: &mut V, _i: &AbiKind) {
    use AbiKind::*;
    match *_i {
        Named(ref _binding_0) => {}
        Default => {}
    }
}
pub fn visit_attr_style<V>(_visitor: &mut V, _i: &AttrStyle) {
    use AttrStyle::*;
    match *_i {
        Outer => {}
        Inner(ref _binding_0) => {}
    }
}
pub fn visit_bare_fn_arg_name<V>(_visitor: &mut V, _i: &BareFnArgName) {
    use BareFnArgName::*;
    match *_i {
        Named(ref _binding_0) => {}
        Wild(ref _binding_0) => {}
    }
}
pub fn visit_bin_op<V>(_visitor: &mut V, _i: &BinOp) {
    use BinOp::*;
    match *_i {
        Add(ref _binding_0) => {}
        Sub(ref _binding_0) => {}
        Mul(ref _binding_0) => {}
        Div(ref _binding_0) => {}
        Rem(ref _binding_0) => {}
        And(ref _binding_0) => {}
        Or(ref _binding_0) => {}
        BitXor(ref _binding_0) => {}
        BitAnd(ref _binding_0) => {}
        BitOr(ref _binding_0) => {}
        Shl(ref _binding_0) => {}
        Shr(ref _binding_0) => {}
        Eq(ref _binding_0) => {}
        Lt(ref _binding_0) => {}
        Le(ref _binding_0) => {}
        Ne(ref _binding_0) => {}
        Ge(ref _binding_0) => {}
        Gt(ref _binding_0) => {}
        AddEq(ref _binding_0) => {}
        SubEq(ref _binding_0) => {}
        MulEq(ref _binding_0) => {}
        DivEq(ref _binding_0) => {}
        RemEq(ref _binding_0) => {}
        BitXorEq(ref _binding_0) => {}
        BitAndEq(ref _binding_0) => {}
        BitOrEq(ref _binding_0) => {}
        ShlEq(ref _binding_0) => {}
        ShrEq(ref _binding_0) => {}
    }
}
pub fn visit_binding_mode<V>(_visitor: &mut V, _i: &BindingMode) {
    use BindingMode::*;
    match *_i {
        ByRef(ref _binding_0, ref _binding_1) => {}
        ByValue(ref _binding_0) => {}
    }
}
pub fn visit_capture_by<V>(_visitor: &mut V, _i: &CaptureBy) {
    use CaptureBy::*;
    match *_i {
        Value(ref _binding_0) => {}
        Ref => {}
    }
}
pub fn visit_constness<V>(_visitor: &mut V, _i: &Constness) {
    use Constness::*;
    match *_i {
        Const(ref _binding_0) => {}
        NotConst => {}
    }
}
pub fn visit_defaultness<V>(_visitor: &mut V, _i: &Defaultness) {
    use Defaultness::*;
    match *_i {
        Default(ref _binding_0) => {}
        Final => {}
    }
}
pub fn visit_expr_kind<V>(_visitor: &mut V, _i: &ExprKind) {
    use ExprKind::*;
    match *_i {
        Box(ref _binding_0) => {}
        InPlace(ref _binding_0) => {}
        Array(ref _binding_0) => {}
        Call(ref _binding_0) => {}
        MethodCall(ref _binding_0) => {}
        Tup(ref _binding_0) => {}
        Binary(ref _binding_0) => {}
        Unary(ref _binding_0) => {}
        Lit(ref _binding_0) => {}
        Cast(ref _binding_0) => {}
        Type(ref _binding_0) => {}
        If(ref _binding_0) => {}
        IfLet(ref _binding_0) => {}
        While(ref _binding_0) => {}
        WhileLet(ref _binding_0) => {}
        ForLoop(ref _binding_0) => {}
        Loop(ref _binding_0) => {}
        Match(ref _binding_0) => {}
        Closure(ref _binding_0) => {}
        Block(ref _binding_0) => {}
        Assign(ref _binding_0) => {}
        AssignOp(ref _binding_0) => {}
        Field(ref _binding_0) => {}
        TupField(ref _binding_0) => {}
        Index(ref _binding_0) => {}
        Range(ref _binding_0) => {}
        Path(ref _binding_0) => {}
        AddrOf(ref _binding_0) => {}
        Break(ref _binding_0) => {}
        Continue(ref _binding_0) => {}
        Ret(ref _binding_0) => {}
        Mac(ref _binding_0) => {}
        Struct(ref _binding_0) => {}
        Repeat(ref _binding_0) => {}
        Paren(ref _binding_0) => {}
        Group(ref _binding_0) => {}
        Try(ref _binding_0) => {}
        Catch(ref _binding_0) => {}
        Yield(ref _binding_0) => {}
    }
}
pub fn visit_fn_arg<V>(_visitor: &mut V, _i: &FnArg) {
    use FnArg::*;
    match *_i {
        SelfRef(ref _binding_0) => {}
        SelfValue(ref _binding_0) => {}
        Captured(ref _binding_0) => {}
        Ignored(ref _binding_0) => {}
    }
}
pub fn visit_foreign_item_kind<V>(_visitor: &mut V, _i: &ForeignItemKind) {
    use ForeignItemKind::*;
    match *_i {
        Fn(ref _binding_0) => {}
        Static(ref _binding_0) => {}
    }
}
pub fn visit_function_ret_ty<V>(_visitor: &mut V, _i: &FunctionRetTy) {
    use FunctionRetTy::*;
    match *_i {
        Default => {}
        Ty(ref _binding_0, ref _binding_1) => {}
    }
}
pub fn visit_impl_item_kind<V>(_visitor: &mut V, _i: &ImplItemKind) {
    use ImplItemKind::*;
    match *_i {
        Const(ref _binding_0) => {}
        Method(ref _binding_0) => {}
        Type(ref _binding_0) => {}
        Macro(ref _binding_0) => {}
    }
}
pub fn visit_impl_polarity<V>(_visitor: &mut V, _i: &ImplPolarity) {
    use ImplPolarity::*;
    match *_i {
        Positive => {}
        Negative(ref _binding_0) => {}
    }
}
pub fn visit_in_place_kind<V>(_visitor: &mut V, _i: &InPlaceKind) {
    use InPlaceKind::*;
    match *_i {
        Arrow(ref _binding_0) => {}
        In(ref _binding_0) => {}
    }
}
pub fn visit_item_kind<V>(_visitor: &mut V, _i: &ItemKind) {
    use ItemKind::*;
    match *_i {
        ExternCrate(ref _binding_0) => {}
        Use(ref _binding_0) => {}
        Static(ref _binding_0) => {}
        Const(ref _binding_0) => {}
        Fn(ref _binding_0) => {}
        Mod(ref _binding_0) => {}
        ForeignMod(ref _binding_0) => {}
        Ty(ref _binding_0) => {}
        Enum(ref _binding_0) => {}
        Struct(ref _binding_0) => {}
        Union(ref _binding_0) => {}
        Trait(ref _binding_0) => {}
        DefaultImpl(ref _binding_0) => {}
        Impl(ref _binding_0) => {}
        Mac(ref _binding_0) => {}
    }
}
pub fn visit_mac_stmt_style<V>(_visitor: &mut V, _i: &MacStmtStyle) {
    use MacStmtStyle::*;
    match *_i {
        Semicolon(ref _binding_0) => {}
        Braces => {}
        NoBraces => {}
    }
}
pub fn visit_meta_item<V>(_visitor: &mut V, _i: &MetaItem) {
    use MetaItem::*;
    match *_i {
        Term(ref _binding_0) => {}
        List(ref _binding_0) => {}
        NameValue(ref _binding_0) => {}
    }
}
pub fn visit_mutability<V>(_visitor: &mut V, _i: &Mutability) {
    use Mutability::*;
    match *_i {
        Mutable(ref _binding_0) => {}
        Immutable => {}
    }
}
pub fn visit_nested_meta_item<V>(_visitor: &mut V, _i: &NestedMetaItem) {
    use NestedMetaItem::*;
    match *_i {
        MetaItem(ref _binding_0) => {}
        Literal(ref _binding_0) => {}
    }
}
pub fn visit_pat<V>(_visitor: &mut V, _i: &Pat) {
    use Pat::*;
    match *_i {
        Wild(ref _binding_0) => {}
        Ident(ref _binding_0) => {}
        Struct(ref _binding_0) => {}
        TupleStruct(ref _binding_0) => {}
        Path(ref _binding_0) => {}
        Tuple(ref _binding_0) => {}
        Box(ref _binding_0) => {}
        Ref(ref _binding_0) => {}
        Lit(ref _binding_0) => {}
        Range(ref _binding_0) => {}
        Slice(ref _binding_0) => {}
        Mac(ref _binding_0) => {}
    }
}
pub fn visit_path_parameters<V>(_visitor: &mut V, _i: &PathParameters) {
    use PathParameters::*;
    match *_i {
        None => {}
        AngleBracketed(ref _binding_0) => {}
        Parenthesized(ref _binding_0) => {}
    }
}
pub fn visit_range_limits<V>(_visitor: &mut V, _i: &RangeLimits) {
    use RangeLimits::*;
    match *_i {
        HalfOpen(ref _binding_0) => {}
        Closed(ref _binding_0) => {}
    }
}
pub fn visit_stmt<V>(_visitor: &mut V, _i: &Stmt) {
    use Stmt::*;
    match *_i {
        Local(ref _binding_0) => {}
        Item(ref _binding_0) => {}
        Expr(ref _binding_0) => {}
        Semi(ref _binding_0, ref _binding_1) => {}
        Mac(ref _binding_0) => {}
    }
}
pub fn visit_trait_bound_modifier<V>(_visitor: &mut V, _i: &TraitBoundModifier) {
    use TraitBoundModifier::*;
    match *_i {
        None => {}
        Maybe(ref _binding_0) => {}
    }
}
pub fn visit_trait_item_kind<V>(_visitor: &mut V, _i: &TraitItemKind) {
    use TraitItemKind::*;
    match *_i {
        Const(ref _binding_0) => {}
        Method(ref _binding_0) => {}
        Type(ref _binding_0) => {}
        Macro(ref _binding_0) => {}
    }
}
pub fn visit_ty<V>(_visitor: &mut V, _i: &Ty) {
    use Ty::*;
    match *_i {
        Slice(ref _binding_0) => {}
        Array(ref _binding_0) => {}
        Ptr(ref _binding_0) => {}
        Rptr(ref _binding_0) => {}
        BareFn(ref _binding_0) => {}
        Never(ref _binding_0) => {}
        Tup(ref _binding_0) => {}
        Path(ref _binding_0) => {}
        TraitObject(ref _binding_0) => {}
        ImplTrait(ref _binding_0) => {}
        Paren(ref _binding_0) => {}
        Group(ref _binding_0) => {}
        Infer(ref _binding_0) => {}
        Mac(ref _binding_0) => {}
    }
}
pub fn visit_ty_param_bound<V>(_visitor: &mut V, _i: &TyParamBound) {
    use TyParamBound::*;
    match *_i {
        Trait(ref _binding_0, ref _binding_1) => {}
        Region(ref _binding_0) => {}
    }
}
pub fn visit_un_op<V>(_visitor: &mut V, _i: &UnOp) {
    use UnOp::*;
    match *_i {
        Deref(ref _binding_0) => {}
        Not(ref _binding_0) => {}
        Neg(ref _binding_0) => {}
    }
}
pub fn visit_unsafety<V>(_visitor: &mut V, _i: &Unsafety) {
    use Unsafety::*;
    match *_i {
        Unsafe(ref _binding_0) => {}
        Normal => {}
    }
}
pub fn visit_variant_data<V>(_visitor: &mut V, _i: &VariantData) {
    use VariantData::*;
    match *_i {
        Struct(ref _binding_0, ref _binding_1) => {}
        Tuple(ref _binding_0, ref _binding_1) => {}
        Unit => {}
    }
}
pub fn visit_view_path<V>(_visitor: &mut V, _i: &ViewPath) {
    use ViewPath::*;
    match *_i {
        Simple(ref _binding_0) => {}
        Glob(ref _binding_0) => {}
        List(ref _binding_0) => {}
    }
}
pub fn visit_visibility<V>(_visitor: &mut V, _i: &Visibility) {
    use Visibility::*;
    match *_i {
        Public(ref _binding_0) => {}
        Crate(ref _binding_0) => {}
        Restricted(ref _binding_0) => {}
        Inherited(ref _binding_0) => {}
    }
}
pub fn visit_where_predicate<V>(_visitor: &mut V, _i: &WherePredicate) {
    use WherePredicate::*;
    match *_i {
        BoundPredicate(ref _binding_0) => {}
        RegionPredicate(ref _binding_0) => {}
        EqPredicate(ref _binding_0) => {}
    }
}
