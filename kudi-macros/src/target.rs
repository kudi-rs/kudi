use proc_macro2::{Span, TokenStream};
use quote::{ToTokens, quote};
use syn::{
    Ident, ItemStruct, TypeGenerics, Visibility,
    parse::{Parse, ParseStream},
};

use crate::utils::{
    ItemUnitStruct, PartialGenerics, PartialImplGenerics, PartialTypeGenerics, PartialWhereClause,
};

pub fn stateful_target(state: ItemState) -> TokenStream {
    let target = Target::from(&state);
    quote!(#target)
}

pub fn stateless_target(item: ItemUnitStruct) -> TokenStream {
    let target = Target::stateless(&item.ident, &item.vis);
    quote!(#target)
}

#[derive(Debug)]
pub struct ItemState {
    item: ItemStruct,
    target: Ident,
}

impl Parse for ItemState {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        let item: ItemStruct = input.parse()?;

        let attrs = item
            .attrs
            .iter()
            .filter(|attr| attr.path().is_ident("target"))
            .collect::<Vec<_>>();
        let target = match &*attrs {
            [target] => target.parse_args::<Ident>(),
            _ => Err(syn::Error::new(
                Span::call_site(),
                "`DepInj` requires one and only one `#[target()]` attribute",
            )),
        }?;

        Ok(Self { item, target })
    }
}

#[derive(Debug)]
struct Target<'a> {
    ident: &'a Ident,
    vis: &'a Visibility,
    state: Option<&'a Ident>,
    impl_gn: Option<PartialImplGenerics<'a>>,
    pt_ty_gn: Option<PartialTypeGenerics<'a>>,
    ty_gn: Option<TypeGenerics<'a>>,
    where_gn: Option<PartialWhereClause<'a>>,
}

impl<'a> Target<'a> {
    fn stateless(ident: &'a Ident, vis: &'a Visibility) -> Target<'a> {
        Self {
            ident,
            vis,
            state: None,
            impl_gn: None,
            pt_ty_gn: None,
            ty_gn: None,
            where_gn: None,
        }
    }

    fn define(&self) -> TokenStream {
        let Self {
            ident,
            vis,
            state,
            impl_gn,
            ty_gn,
            where_gn,
            ..
        } = self;

        let repr_transparent = state.map(|_| quote!(#[repr(transparent)]));
        let marker = state.map(|_| quote!(_marker: ::core::marker::PhantomData<#state #ty_gn>,));
        let where_gn = where_gn.map(|gn| quote!(where #gn));

        quote! {
            #[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
            #repr_transparent
            #vis struct #ident<#impl_gn __Deps__: ?Sized>
            #where_gn
            {
                #marker
                deps: __Deps__,
            }
        }
    }

    fn impl_cast(&self) -> TokenStream {
        let Self {
            ident,
            state,
            impl_gn,
            pt_ty_gn,
            ty_gn,
            where_gn,
            ..
        } = self;

        let marker = state.map(|_| quote!(_marker: ::core::marker::PhantomData::<#state #ty_gn>,));
        let where_gn = where_gn.map(|gn| quote!(where #gn));

        quote! {
            impl<#impl_gn __Deps__> #ident<#pt_ty_gn __Deps__>
            #where_gn
            {
                #[inline]
                pub fn inj(deps: __Deps__) -> Self {
                    Self {
                        #marker
                        deps,
                    }
                }

                #[inline]
                pub fn prj(self) -> __Deps__ {
                    self.deps
                }
            }
        }
    }

    fn impl_ref_cast(&self) -> TokenStream {
        let Self {
            ident,
            impl_gn,
            pt_ty_gn,
            where_gn,
            ..
        } = self;

        quote! {
            impl<#impl_gn __Deps__> #ident<#pt_ty_gn __Deps__>
            where
                __Deps__: ?Sized,
                #where_gn
            {
                #[inline]
                pub fn inj_ref(deps: &__Deps__) -> &Self {
                     unsafe { &*(deps as *const __Deps__ as *const Self) }
                }

                #[inline]
                pub fn prj_ref(&self) -> &__Deps__ {
                    unsafe { &*(self as *const Self as *const __Deps__) }
                }

                #[inline]
                pub fn inj_ref_mut(deps: &mut __Deps__) -> &mut Self {
                    unsafe { &mut*(deps as *mut __Deps__ as *mut Self) }
                }

                #[inline]
                pub fn prj_ref_mut(&mut self) -> &mut __Deps__ {
                    unsafe { &mut*(self as *mut Self as *mut __Deps__) }
                }

                #[inline]
                pub fn inj_box(deps: Box<__Deps__>) -> Box<Self> {
                    unsafe { Box::from_raw(Box::into_raw(deps) as *mut Self) }
                }

                #[inline]
                pub fn prj_box(self: Box<Self>) -> Box<__Deps__> {
                    unsafe { Box::from_raw(Box::into_raw(self) as *mut __Deps__) }
                }

                #[inline]
                pub fn inj_rc(deps: ::std::rc::Rc<__Deps__>) -> ::std::rc::Rc<Self> {
                    unsafe { ::std::rc::Rc::from_raw(::std::rc::Rc::into_raw(deps) as *const Self)}
                }

                #[inline]
                pub fn prj_rc(self: ::std::rc::Rc<Self>) -> ::std::rc::Rc<__Deps__> {
                    unsafe { ::std::rc::Rc::from_raw(::std::rc::Rc::into_raw(self) as *const __Deps__) }
                }

                #[inline]
                pub fn inj_arc(deps: ::std::sync::Arc<__Deps__>) -> ::std::sync::Arc<Self> {
                    unsafe { ::std::sync::Arc::from_raw(::std::sync::Arc::into_raw(deps) as *const Self)}
                }

                #[inline]
                pub fn prj_arc(self: ::std::sync::Arc<Self>) -> ::std::sync::Arc<__Deps__> {
                    unsafe { ::std::sync::Arc::from_raw(::std::sync::Arc::into_raw(self) as *const __Deps__) }
                }

                #[inline]
                pub fn inj_pin_ref(deps: ::core::pin::Pin<&__Deps__>) -> ::core::pin::Pin<&Self> {
                     unsafe {
                        ::core::pin::Pin::new_unchecked(
                            &*(::core::pin::Pin::into_inner_unchecked(deps) as *const __Deps__ as *const Self)
                        )
                    }
                }

                #[inline]
                pub fn prj_pin_ref(self: ::core::pin::Pin<&Self>) -> ::core::pin::Pin<&__Deps__> {
                    unsafe {
                        ::core::pin::Pin::new_unchecked(
                            &*(::core::pin::Pin::into_inner_unchecked(self) as *const Self as *const __Deps__)
                        )
                    }
                }

                #[inline]
                pub fn inj_pin_ref_mut(deps: ::core::pin::Pin<&mut __Deps__>) -> ::core::pin::Pin<&mut Self> {
                    unsafe {
                        ::core::pin::Pin::new_unchecked(
                            &mut*(::core::pin::Pin::into_inner_unchecked(deps) as *mut __Deps__ as *mut Self)
                        )
                    }
                }

                #[inline]
                pub fn prj_pin_ref_mut(self: ::core::pin::Pin<&mut Self>) -> ::core::pin::Pin<&mut __Deps__> {
                    unsafe {
                        ::core::pin::Pin::new_unchecked(
                            &mut*(::core::pin::Pin::into_inner_unchecked(self) as *mut Self as *mut __Deps__)
                        )
                    }
                }

                #[inline]
                pub fn inj_pin_box(deps: ::core::pin::Pin<Box<__Deps__>>) -> ::core::pin::Pin<Box<Self>> {
                    unsafe {
                        ::core::pin::Pin::new_unchecked(
                            Box::from_raw(Box::into_raw(::core::pin::Pin::into_inner_unchecked(deps)) as *mut Self)
                        )
                    }
                }

                #[inline]
                pub fn prj_pin_box(self: ::core::pin::Pin<Box<Self>>) -> ::core::pin::Pin<Box<__Deps__>> {
                    unsafe {
                        ::core::pin::Pin::new_unchecked(
                            Box::from_raw(Box::into_raw(::core::pin::Pin::into_inner_unchecked(self)) as *mut __Deps__)
                        )
                    }
                }

                #[inline]
                pub fn inj_pin_rc(deps: ::core::pin::Pin<::std::rc::Rc<__Deps__>>) -> ::core::pin::Pin<::std::rc::Rc<Self>> {
                    unsafe {
                        ::core::pin::Pin::new_unchecked(
                            ::std::rc::Rc::from_raw(::std::rc::Rc::into_raw(::core::pin::Pin::into_inner_unchecked(deps)) as *const Self)
                        )
                    }
                }

                #[inline]
                pub fn prj_pin_rc(self: ::core::pin::Pin<::std::rc::Rc<Self>>) -> ::core::pin::Pin<::std::rc::Rc<__Deps__>> {
                    unsafe {
                        ::core::pin::Pin::new_unchecked(
                            ::std::rc::Rc::from_raw(::std::rc::Rc::into_raw(::core::pin::Pin::into_inner_unchecked(self)) as *const __Deps__)
                        )
                    }
                }

                #[inline]
                pub fn inj_pin_arc(deps: ::core::pin::Pin<::std::sync::Arc<__Deps__>>) -> ::core::pin::Pin<::std::sync::Arc<Self>> {
                    unsafe {
                        ::core::pin::Pin::new_unchecked(
                            ::std::sync::Arc::from_raw(::std::sync::Arc::into_raw(::core::pin::Pin::into_inner_unchecked(deps)) as *const Self)
                        )
                    }
                }

                #[inline]
                pub fn prj_pin_arc(self: ::core::pin::Pin<::std::sync::Arc<Self>>) -> ::core::pin::Pin<::std::sync::Arc<__Deps__>> {
                    unsafe {
                        ::core::pin::Pin::new_unchecked(
                            ::std::sync::Arc::from_raw(::std::sync::Arc::into_raw(::core::pin::Pin::into_inner_unchecked(self)) as *const __Deps__)
                        )
                    }
                }
            }
        }
    }

    fn impls_convert(&self) -> TokenStream {
        let Self {
            ident,
            state,
            impl_gn,
            pt_ty_gn,
            ty_gn,
            where_gn,
            ..
        } = self;

        quote! {
            impl<#impl_gn __Deps__> ::std::ops::Deref for #ident<#pt_ty_gn __Deps__>
            where
                __Deps__: AsRef<#state #ty_gn> + ?Sized,
                #where_gn
            {
                type Target = #state #ty_gn;

                #[inline]
                fn deref(&self) -> &Self::Target {
                    self.deps.as_ref()
                }
            }

            impl<#impl_gn __Deps__> ::std::ops::DerefMut for #ident<#pt_ty_gn __Deps__>
            where
                __Deps__: AsRef<#state #ty_gn> + AsMut<#state #ty_gn> + ?Sized,
                #where_gn
            {
                #[inline]
                fn deref_mut(&mut self) -> &mut Self::Target {
                    self.deps.as_mut()
                }
            }

            impl<#impl_gn __Deps__> From<#ident<#pt_ty_gn __Deps__>> for #state #ty_gn
            where
                __Deps__: Into<#state #ty_gn>,
                #where_gn
            {
                #[inline]
                fn from(target: #ident<#pt_ty_gn __Deps__>) -> Self {
                    target.prj().into()
                }
            }
        }
    }
}

impl ToTokens for Target<'_> {
    fn to_tokens(&self, tokens: &mut TokenStream) {
        tokens.extend(self.define());
        tokens.extend(self.impl_cast());
        tokens.extend(self.impl_ref_cast());
        if self.state.is_some() {
            tokens.extend(self.impls_convert());
        }
    }
}

impl<'a> From<&'a ItemState> for Target<'a> {
    fn from(state: &'a ItemState) -> Self {
        let ItemState { item, target } = state;

        let (_, ty_gn, where_gn) = item.generics.split_for_impl();
        let (impl_gn, pt_ty_gn) = PartialGenerics(&item.generics.params).split_for_impl();
        let where_gn = where_gn.map(Into::into);

        Self {
            ident: target,
            vis: &item.vis,
            state: Some(&item.ident),
            impl_gn: Some(impl_gn),
            pt_ty_gn: Some(pt_ty_gn),
            ty_gn: Some(ty_gn),
            where_gn,
        }
    }
}
