use syn::{
    Expr, Ident, ImplItem, ImplItemConst, ImplItemFn, ImplItemType, Path, ReturnType, TraitBound,
    Type, TypeArray, TypeBareFn, TypeGroup, TypeImplTrait, TypeParamBound, TypeParen, TypePtr,
    TypeReference, TypeTuple,
};

use super::ast::ItemImplTrait;

pub trait Isomorphic {
    fn is_isomorphic(ctn: &Self, target: &Self, ctx: &CompareCtx) -> bool;
}

#[derive(Debug)]
pub struct CompareCtx<'a> {
    trait_: &'a Path,
    target: &'a Type,
}

impl Isomorphic for ItemImplTrait {
    fn is_isomorphic(ctn: &Self, target: &Self, ctx: &CompareCtx) -> bool {
        if ctn.items.len() != target.items.len() {
            return false;
        }

        for (lhs, rhs) in ctn.items.iter().zip(&ctn.items) {
            if !Isomorphic::is_isomorphic(lhs, rhs, ctx) {
                return false;
            }
        }

        true
    }
}

impl Isomorphic for ImplItem {
    fn is_isomorphic(ctn: &Self, target: &Self, ctx: &CompareCtx) -> bool {
        match (ctn, target) {
            (ImplItem::Const(lhs), ImplItem::Const(rhs)) => {
                Isomorphic::is_isomorphic(lhs, rhs, ctx)
            }
            (ImplItem::Fn(lhs), ImplItem::Fn(rhs)) => Isomorphic::is_isomorphic(lhs, rhs, ctx),
            (ImplItem::Type(lhs), ImplItem::Type(rhs)) => Isomorphic::is_isomorphic(lhs, rhs, ctx),
            (ImplItem::Macro(_), ImplItem::Macro(_))
            | (ImplItem::Verbatim(_), ImplItem::Verbatim(_)) => true,
            _ => false,
        }
    }
}

impl Isomorphic for ImplItemConst {
    fn is_isomorphic(ctn: &Self, target: &Self, ctx: &CompareCtx) -> bool {
        let v = Isomorphic::is_isomorphic(&ctn.ident, &target.ident, ctx)
            && Isomorphic::is_isomorphic(&ctn.ty, &target.ty, ctx);

        let v_expr = if let Expr::Path(expr) = &ctn.expr {
            if let Some(qself) = expr.qself.as_ref().filter(|qs| qs.as_token.is_some()) {
                let v_target = Isomorphic::is_isomorphic(qself.ty.as_ref(), ctx.target, ctx);
                let v_trait = expr.path.segments[qself.position - 1].ident
                    == ctx.trait_.segments.last().unwrap().ident;
                let v_ident = Isomorphic::is_isomorphic(
                    &expr.path.segments[qself.position].ident,
                    &target.ident,
                    ctx,
                );
                v_target && v_trait && v_ident
            } else {
                false
            }
        } else {
            false
        };

        v && v_expr
    }
}

impl Isomorphic for ImplItemFn {
    fn is_isomorphic(ctn: &Self, target: &Self, ctx: &CompareCtx) -> bool {
        todo!()
    }
}

// WARN: we don't support checking type generics at present
impl Isomorphic for ImplItemType {
    fn is_isomorphic(ctn: &Self, target: &Self, ctx: &CompareCtx) -> bool {
        let v = Isomorphic::is_isomorphic(&ctn.ident, &target.ident, ctx);

        let v_ty = if let Type::Path(ty) = &ctn.ty {
            if let Some(qself) = ty.qself.as_ref().filter(|qs| qs.as_token.is_some()) {
                let v_target = Isomorphic::is_isomorphic(qself.ty.as_ref(), ctx.target, ctx);
                let v_trait = ty.path.segments[qself.position - 1].ident
                    == ctx.trait_.segments.last().unwrap().ident;
                let v_ident = Isomorphic::is_isomorphic(
                    &ty.path.segments[qself.position].ident,
                    &target.ident,
                    ctx,
                );
                v_target && v_trait && v_ident
            } else {
                false
            }
        } else {
            false
        };

        v && v_ty
    }
}

impl Isomorphic for Ident {
    fn is_isomorphic(ctn: &Self, target: &Self, ctx: &CompareCtx) -> bool {
        ctn == target
    }
}

impl Isomorphic for Type {
    fn is_isomorphic(ctn: &Self, target: &Self, ctx: &CompareCtx) -> bool {
        match (ctn, target) {
            (Type::Array(lhs), Type::Array(rhs)) => Isomorphic::is_isomorphic(lhs, rhs, ctx),
            (Type::BareFn(lhs), Type::BareFn(rhs)) => Isomorphic::is_isomorphic(lhs, rhs, ctx),
            (Type::Group(lhs), Type::Group(rhs)) => Isomorphic::is_isomorphic(lhs, rhs, ctx),
            (Type::ImplTrait(lhs), Type::ImplTrait(rhs)) => {
                Isomorphic::is_isomorphic(lhs, rhs, ctx)
            }
            (Type::Paren(lhs), Type::Paren(rhs)) => Isomorphic::is_isomorphic(lhs, rhs, ctx),
            (Type::Path(lhs), Type::Path(rhs)) => Isomorphic::is_isomorphic(lhs, rhs, ctx),
            (Type::Ptr(lhs), Type::Ptr(rhs)) => Isomorphic::is_isomorphic(lhs, rhs, ctx),
            (Type::Reference(lhs), Type::Reference(rhs)) => {
                Isomorphic::is_isomorphic(lhs, rhs, ctx)
            }
            (Type::Slice(lhs), Type::Slice(rhs)) => Isomorphic::is_isomorphic(lhs, rhs, ctx),
            (Type::TraitObject(lhs), Type::TraitObject(rhs)) => {
                Isomorphic::is_isomorphic(lhs, rhs, ctx)
            }
            (Type::Tuple(lhs), Type::Tuple(rhs)) => Isomorphic::is_isomorphic(lhs, rhs, ctx),
            (Type::Never(_), Type::Never(_))
            | (Type::Infer(_), Type::Infer(_))
            | (Type::Macro(_), Type::Macro(_))
            | (Type::Verbatim(_), Type::Verbatim(_)) => true, // ignore
            _ => false,
        }
    }
}

impl Isomorphic for TypeArray {
    fn is_isomorphic(ctn: &Self, target: &Self, ctx: &CompareCtx) -> bool {
        Type::is_isomorphic(&ctn.elem, &target.elem, ctx)
            && Expr::is_isomorphic(&ctn.len, &target.len, ctx)
    }
}

impl Isomorphic for TypeBareFn {
    fn is_isomorphic(ctn: &Self, target: &Self, ctx: &CompareCtx) -> bool {
        if ctn.inputs.len() != target.inputs.len() {
            return false;
        }

        for (lhs, rhs) in ctn.inputs.iter().zip(&target.inputs) {
            if !Isomorphic::is_isomorphic(&lhs.ty, &rhs.ty, ctx) {
                return false;
            }
        }

        Isomorphic::is_isomorphic(&ctn.output, &target.output, ctx)
    }
}

impl Isomorphic for ReturnType {
    fn is_isomorphic(ctn: &Self, target: &Self, ctx: &CompareCtx) -> bool {
        match (ctn, target) {
            (ReturnType::Default, ReturnType::Default) => true,
            (ReturnType::Type(_, lhs), ReturnType::Type(_, rhs)) => {
                Isomorphic::is_isomorphic(lhs.as_ref(), rhs.as_ref(), ctx)
            }
            _ => false,
        }
    }
}

impl Isomorphic for TypeGroup {
    fn is_isomorphic(ctn: &Self, target: &Self, ctx: &CompareCtx) -> bool {
        Isomorphic::is_isomorphic(ctn.elem.as_ref(), &target.elem.as_ref(), ctx)
    }
}

impl Isomorphic for TypeParen {
    fn is_isomorphic(ctn: &Self, target: &Self, ctx: &CompareCtx) -> bool {
        Isomorphic::is_isomorphic(ctn.elem.as_ref(), &target.elem.as_ref(), ctx)
    }
}

// WARN: we don't support checking lifetime at present
impl Isomorphic for TypeReference {
    fn is_isomorphic(ctn: &Self, target: &Self, ctx: &CompareCtx) -> bool {
        ctn.mutability == target.mutability
            && Isomorphic::is_isomorphic(ctn.elem.as_ref(), &target.elem.as_ref(), ctx)
    }
}

impl Isomorphic for TypePtr {
    fn is_isomorphic(ctn: &Self, target: &Self, ctx: &CompareCtx) -> bool {
        if (ctn.const_token == target.const_token) || (ctn.mutability == target.mutability) {
            Isomorphic::is_isomorphic(ctn.elem.as_ref(), &target.elem.as_ref(), ctx)
        } else {
            false
        }
    }
}

impl Isomorphic for TypeTuple {
    fn is_isomorphic(ctn: &Self, target: &Self, ctx: &CompareCtx) -> bool {
        if ctn.elems.len() != target.elems.len() {
            return false;
        }

        for (lhs, rhs) in ctn.elems.iter().zip(&target.elems) {
            if !Isomorphic::is_isomorphic(lhs, rhs, ctx) {
                return false;
            }
        }

        true
    }
}

// WARN: we don't support checking lifetime at present
impl Isomorphic for TypeImplTrait {
    fn is_isomorphic(ctn: &Self, target: &Self, ctx: &CompareCtx) -> bool {
        if ctn.bounds.len() != target.bounds.len() {
            return false;
        }

        fn f(bound: &TypeParamBound) -> Option<&TraitBound> {
            if let TypeParamBound::Trait(t) = bound {
                Some(t)
            } else {
                None
            }
        }

        for (lhs, rhs) in ctn
            .bounds
            .iter()
            .filter_map(f)
            .zip(target.bounds.iter().filter_map(f))
        {
            let v_mod = lhs.modifier == rhs.modifier;
            let v_trait = Isomorphic::is_isomorphic(&lhs.path, &rhs.path, ctx);
            if !(v_mod && v_trait) {
                return false;
            }
        }

        true
    }
}
