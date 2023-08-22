use inflector::Inflector;
use proc_macro2::{Span, TokenStream};
use proc_macro_error::{abort, emit_error, emit_warning};
use quote::{quote, quote_spanned};
use syn::{
    parse::Parse, spanned::Spanned, FnArg, GenericParam, Ident, ImplItem, ImplItemFn, ItemImpl,
    Lifetime, LifetimeParam, Token,
};

extern crate proc_macro;

enum HostObjectHelper {
    Getter {
        prop: Option<Ident>,
    },
    Setter {
        prop: Option<Ident>,
    },
    Method {
        name: Option<Ident>,
    },
    #[allow(dead_code)]
    Include,
}

impl Parse for HostObjectHelper {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let id: Ident = input.parse()?;

        let out = match id.to_string().as_ref() {
            "getter" => Self::Getter {
                prop: if input.peek(Token![as]) {
                    input.parse::<Token![as]>()?;
                    input.parse()?
                } else if input.is_empty() {
                    None
                } else {
                    abort!(
                        id,
                        "invalid helper attribute, unexpected token in arguments"
                    )
                },
            },
            "setter" => Self::Setter {
                prop: if input.peek(Token![as]) {
                    input.parse::<Token![as]>()?;
                    input.parse()?
                } else if input.is_empty() {
                    None
                } else {
                    abort!(
                        id,
                        "invalid helper attribute, unexpected token in arguments"
                    )
                },
            },
            "method" => Self::Method {
                name: if input.peek(Token![as]) {
                    input.parse::<Token![as]>()?;
                    input.parse()?
                } else if input.is_empty() {
                    None
                } else {
                    abort!(
                        id,
                        "invalid helper attribute, unexpected token in arguments"
                    )
                },
            },
            "include" => {
                abort!(id, "#[host_object(include)] does not work properly yet");

                // if input.is_empty() {
                //     Self::Include
                // } else {
                //     abort!(id, "invalid helper attribute, unexpected token in arguments")
                // }
            }
            _ => abort!(id, "invalid helper attribute"),
        };

        Ok(out)
    }
}

struct HostObjectGetter {
    prop: String,
    method: ImplItemFn,
}

struct HostObjectSetter {
    prop: String,
    method: ImplItemFn,
}

struct HostObjectMethod {
    name: String,
    method: ImplItemFn,
}

struct HostObjectInclude {
    method: ImplItemFn,
}

pub struct HostObjectImpl(pub TokenStream);

impl Parse for HostObjectImpl {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let mut impl_block: ItemImpl = input.parse()?;

        if let Some((_, ty, _)) = &impl_block.trait_ {
            emit_error!(ty, "#[host_object] should not be used on trait impls");
        }

        let mut getters = vec![];
        let mut setters = vec![];
        let mut methods = vec![];
        let mut includes = vec![];

        for it in &mut impl_block.items {
            match it {
                ImplItem::Fn(it) => {
                    let mut helper: Option<HostObjectHelper> = None;

                    it.attrs = it
                        .attrs
                        .iter()
                        .map(|attr| {
                            if !attr.path().is_ident("host_object") {
                                return Ok(Some(attr.clone()));
                            }

                            if let None = helper {
                                helper = Some(attr.parse_args()?);
                            } else {
                                emit_warning!(attr, "only the first helper attribute is used");
                            }

                            syn::Result::<_>::Ok(None)
                        })
                        .collect::<Result<Vec<_>, _>>()?
                        .into_iter()
                        .filter_map(|i| i)
                        .collect();

                    match helper.unwrap_or(HostObjectHelper::Method { name: None }) {
                        HostObjectHelper::Getter { prop } => {
                            let prop = if let Some(prop) = prop {
                                prop.to_string()
                            } else {
                                let prop = it.sig.ident.to_string();
                                let prop = prop.strip_prefix("get_").unwrap_or(prop.as_str());
                                prop.to_camel_case()
                            };

                            if let Some(a) = it.sig.asyncness {
                                emit_error!(a, "getters cannot be async");
                                continue;
                            }

                            let inputs: Vec<_> = it.sig.inputs.iter().collect();

                            // just check the shape, the actual types will be
                            // checked by the compiler after the macro expands

                            let input_valid = match &inputs[..] {
                                [FnArg::Receiver(_), FnArg::Typed(_)] => true,
                                _ => false,
                            };

                            let output_valid = match &it.sig.output {
                                syn::ReturnType::Default => false,
                                syn::ReturnType::Type(_, _) => true,
                            };

                            if !(input_valid && output_valid) {
                                emit_error!(it.sig.inputs, "getters should be fn(&self, rt: &mut RuntimeHandle) -> Result<T> where T: IntoValue");
                                continue;
                            }

                            getters.push(HostObjectGetter {
                                prop,
                                method: it.clone(),
                            })
                        }
                        HostObjectHelper::Setter { prop } => {
                            let prop = if let Some(prop) = prop {
                                prop.to_string()
                            } else {
                                let prop = it.sig.ident.to_string();
                                let prop = prop.strip_prefix("set_").unwrap_or(prop.as_str());
                                prop.to_camel_case()
                            };

                            if let Some(a) = it.sig.asyncness {
                                emit_error!(a, "setters cannot be async");
                                continue;
                            }

                            let inputs: Vec<_> = it.sig.inputs.iter().collect();

                            // just check the shape, the actual types will be
                            // checked by the compiler after the macro expands

                            let input_valid = match &inputs[..] {
                                [FnArg::Receiver(_), FnArg::Typed(_), FnArg::Typed(_)] => true,
                                _ => false,
                            };

                            let output_valid = match &it.sig.output {
                                syn::ReturnType::Default => false,
                                syn::ReturnType::Type(_, _) => true,
                            };

                            if !(input_valid && output_valid) {
                                emit_error!(it.sig.inputs, "setters should be fn(&self, rt: &mut RuntimeHandle, value: JsiValue) -> Result<()>");
                                continue;
                            }

                            setters.push(HostObjectSetter {
                                prop,
                                method: it.clone(),
                            })
                        }
                        HostObjectHelper::Method { name } => {
                            let name = if let Some(name) = name {
                                name.to_string()
                            } else {
                                let name = it.sig.ident.to_string();
                                name.to_camel_case()
                            };

                            let inputs: Vec<_> = it.sig.inputs.iter().collect();

                            // just check the shape, the actual types will be
                            // checked by the compiler after the macro expands

                            let input_valid = match &inputs[..] {
                                [FnArg::Receiver(_), FnArg::Typed(_), ..] => true,
                                _ => false,
                            };

                            let output_valid = match &it.sig.output {
                                syn::ReturnType::Default => false,
                                syn::ReturnType::Type(_, _) => true,
                            };

                            if !(input_valid && output_valid) {
                                emit_error!(it.sig.inputs, "methods should be fn(&self, rt: &mut RuntimeHandle, ...) -> Result<T>");
                                continue;
                            }

                            methods.push(HostObjectMethod {
                                name,
                                method: it.clone(),
                            })
                        }
                        HostObjectHelper::Include => {
                            if let Some(a) = it.sig.asyncness {
                                emit_error!(a, "includes cannot be async");
                                continue;
                            }

                            let inputs: Vec<_> = it.sig.inputs.iter().collect();

                            // just check the shape, the actual types will be
                            // checked by the compiler after the macro expands

                            let input_valid = match &inputs[..] {
                                [FnArg::Receiver(r)] => r.mutability.is_some(),
                                _ => false,
                            };

                            let output_valid = match &it.sig.output {
                                syn::ReturnType::Default => false,
                                syn::ReturnType::Type(_, ty) => match &**ty {
                                    syn::Type::Reference(r) => r.mutability.is_some(),
                                    _ => false,
                                },
                            };

                            if !(input_valid && output_valid) {
                                emit_error!(
                                    it.sig.inputs,
                                    "includes should be fn(&mut self) -> &mut T where T: UserHostObject"
                                );
                                continue;
                            }

                            includes.push(HostObjectInclude { method: it.clone() })
                        }
                    }
                }
                it => emit_error!(
                    it,
                    "#[host_object] impl blocks should only contain methods";
                    note = "you can put other items in a separate impl block"
                ),
            }
        }

        let ty = &impl_block.self_ty;
        let impl_block = &impl_block;

        let prop_names: Vec<_> = getters
            .iter()
            .map(|g| g.prop.clone())
            .chain(setters.iter().map(|s| s.prop.clone()))
            .chain(methods.iter().map(|m| m.name.clone()))
            .collect();

        let getter_matchers = getters.into_iter().map(|getter| {
            let js_getter_name = getter.prop;

            let method = getter.method;
            let method_span = method.sig.span();
            let method_name = method.sig.ident;

            quote_spanned! {method_span=>
                #js_getter_name => {
                    Ok(::jsi::IntoValue::into_value(self.#method_name(rt)?, rt))
                }
            }
        });

        let setter_matchers = setters.into_iter().map(|setter| {
            let js_setter_name = setter.prop;

            let method = setter.method;
            let method_span = method.sig.span();
            let method_name = method.sig.ident;

            quote_spanned! {method_span=>
                #js_setter_name => {
                    self.#method_name(rt, anyhow::Context::context(jsi::FromValue::from_value(value, rt), "tried to set property to invalid value")?)?;
                    Ok(())
                }
            }
        });

        let method_matchers = methods.into_iter().map(|method| {
            let js_method_name = method.name;

            let method_span = method.method.sig.span();
            let method_name = method.method.sig.ident;

            // subtract 2 to exclude &self and runtime handle
            let (arg_names, arg_types): (Vec<_>, Vec<_>) = method.method.sig.inputs
                .into_iter()
                .skip(2)
                .map(|f| {
                    match f {
                        // we already checked that the method signature is correct
                        FnArg::Receiver(_) => unreachable!(),
                        FnArg::Typed(f) => (*f.pat, *f.ty),
                    }
                })
                .unzip();

            let arg_count = arg_names.len();

            let retval = if method.method.sig.asyncness.is_some() {
                let trace = if cfg!(feature = "host-object-trace") {
                    quote! {
                        ::log::trace!("using this = {:p} for async method in {}", this, ::std::any::type_name::<Self>());
                    }
                } else {
                    quote! {}
                };

                quote_spanned! {method_span=>
                    // assert that `this` is an instance of Self but don't hold
                    // onto the reference, instead move the
                    // JsiSharedUserHostObject into the closure and then get the
                    // reference again
                    let _ = anyhow::Context::context(this.get_inner::<Self>(), "this is not bound correctly")?;

                    let promise = ::jsi::create_promise(
                        move |
                            resolve: ::jsi::JsiFn,
                            reject: ::jsi::JsiFn,
                            _rt: &mut ::jsi::RuntimeHandle,
                        | {
                            let this = this;

                            // Arc<Mutex> to make type system happy since JsiValues
                            // aren't Sync; inexpensive b/c we will only access each
                            // function at most once
                            let resolve = ::std::sync::Arc::new(::std::sync::Mutex::new(resolve));
                            let reject = ::std::sync::Arc::new(::std::sync::Mutex::new(reject));

                            splicer_js_api::spawn(Box::new(move |rt| Box::pin(async move {
                                let this = this;
                                let this = this.get_inner::<Self>().unwrap();
                                #trace
                                let res = this.#method_name(rt, #(#arg_names),*).await;

                                match res {
                                    Ok(val) => {
                                        let val = ::jsi::IntoValue::into_value(val, rt);
                                        
                                        resolve.lock().unwrap().call(::std::iter::once(val), rt)?;

                                        // <Self as ::jsi::AsyncUserHostObject>::invoke(Box::new(move || {
                                        //     resolve.lock().unwrap().call(::std::iter::once(val), rt)?;
                                        //     Ok(())
                                        // }));
                                    },
                                    Err(err) => {
                                        reject.lock().unwrap().call(::std::iter::empty(), rt);

                                        // for some reason, just calling reject() doesn't actually cause
                                        // the promise to reject, so instead we create a rejected
                                        // promise and return that

                                        let promise_ctor = rt.global().get(::jsi::PropName::new("Promise", rt), rt);
                                        let promise_ctor: ::jsi::JsiObject = ::jsi::FromValue::from_value(
                                            &promise_ctor, 
                                            rt,
                                        ).expect("Promise constructor is not an object");
                                        let promise_reject: ::jsi::JsiFn = ::jsi::FromValue::from_value(
                                            &promise_ctor.get(::jsi::PropName::new("reject", rt), rt), 
                                            rt,
                                        ).expect("Promise.reject is not a function");

                                        let err = ::jsi::js_error!(err, rt);

                                        let rejection = promise_reject.call(::std::iter::once(err.into_value(rt)), rt).unwrap();
                                        reject.lock().unwrap().call(::std::iter::once(rejection), rt)?;

                                        // <Self as ::jsi::AsyncUserHostObject>::invoke(Box::new(move || {
                                        //     reject.lock().unwrap().call(::std::iter::empty());

                                        //     // for some reason, just calling reject() doesn't actually cause
                                        //     // the promise to reject, so instead we create a rejected
                                        //     // promise and return that

                                        //     let promise_ctor = rt.global().get(::jsi::PropName::new("Promise", rt), rt);
                                        //     let promise_ctor: ::jsi::JsiObject = ::jsi::FromValue::from_value(
                                        //         &promise_ctor, 
                                        //         rt,
                                        //     ).expect("Promise constructor is not an object");
                                        //     let promise_reject: ::jsi::JsiFn = ::jsi::FromValue::from_value(
                                        //         &promise_ctor.get(::jsi::PropName::new("reject", rt), rt), 
                                        //         rt,
                                        //     ).expect("Promise.reject is not a function");

                                        //     let err = ::jsi::js_error!(err, rt);

                                        //     let rejection = promise_reject.call(::std::iter::once(err.into())).unwrap();
                                        //     reject.lock().unwrap().call(::std::iter::once(rejection))?;

                                        //     Ok(())
                                        // }));
                                    },
                                }

                                Ok(())
                            })));
                        },
                        rt
                    );

                    Ok(::jsi::IntoValue::into_value(promise, rt))
                }
            } else {
                let trace = if cfg!(feature = "host-object-trace") {
                    quote! {
                        ::log::trace!("using this = {:p} for sync method in {}", this, ::std::any::type_name::<Self>());
                    }
                } else {
                    quote! {}
                };

                quote_spanned! {method_span=>
                    let this = anyhow::Context::context(this.get_inner::<Self>(), "this is not bound correctly")?;
                    #trace
                    Ok(::jsi::IntoValue::into_value(this.#method_name(rt, #(#arg_names),*)?, rt))
                }
            };

            let args_iter = if arg_count > 0 {
                quote! { let mut _args = args.into_iter(); }
            } else {
                quote! { }
            };

            quote! {
                #js_method_name => {
                    Ok(::jsi::IntoValue::into_value(
                            ::jsi::JsiFn::from_host_fn(
                            &PropName::new(#js_method_name, rt),
                            #arg_count,
                            Box::new(move |this, args, rt| {
                                let this: ::jsi::SharedJsiUserHostObject =
                                    anyhow::Context::context(
                                        ::jsi::FromValue::from_value(&this, rt), 
                                        "this is not bound correctly"
                                    )?;

                                #args_iter

                                #(
                                    let #arg_names: #arg_types = match _args.next() {
                                        Some(arg) => if let Some(arg) = ::jsi::FromValue::from_value(&arg, rt) {
                                            arg
                                        } else {
                                            ::anyhow::bail!("argument {} could not be converted to {}", stringify!($arg_names), std::any::type_name::<#arg_types>())
                                        },
                                        None => {
                                            ::anyhow::bail!("not enough arguments")
                                        }
                                    };
                                )*

                                #retval
                            }),
                            rt,
                        ),
                        rt
                    ))
                }
            }
        });

        let include_getter_matchers = includes.iter().map(|include| {
            let method = &include.method;
            let method_name = method.sig.ident.clone();

            quote! {
                let incl = self.#method_name();
                let val = ::jsi::UserHostObject::get(incl, rt, name.clone())?;
                if !val.is_undefined() {
                    return Ok(val)
                }
            }
        });

        let include_setter_matchers = includes.iter().map(|include| {
            let method = &include.method;
            let method_name = method.sig.ident.clone();

            quote! {
                let incl = self.#method_name();
                let res = ::jsi::UserHostObject::set(incl, rt, name.clone(), value.clone());
                if res.is_ok() {
                    return Ok(())
                }
            }
        });

        let include_prop_names = includes.iter().map(|include| {
            let method = &include.method;
            let method_name = method.sig.ident.clone();

            quote! {
                let incl = self.#method_name();
                props.extend(incl.properties(rt));
            }
        });

        let mut generic_params = impl_block.generics.params.clone();

        let has_rt_lifetime = if let Some(GenericParam::Lifetime(lt)) = generic_params.first() {
            lt.lifetime.ident == "rt"
        } else {
            false
        };

        if !has_rt_lifetime {
            generic_params.insert(
                0,
                GenericParam::Lifetime(LifetimeParam::new(Lifetime::new("'rt", Span::call_site()))),
            );
        }

        Ok(Self(quote! {
            #impl_block

            impl<#generic_params> ::jsi::UserHostObject<'rt> for #ty {
                fn get(
                  &mut self,
                  name: ::jsi::PropName<'rt>,
                  rt: &mut ::jsi::RuntimeHandle<'rt>,
                ) -> anyhow::Result<::jsi::JsiValue<'rt>> {
                    match rt.to_string(&name).as_str() {
                        #(#getter_matchers)*
                        #(#method_matchers)*
                        _ => {
                            #(#include_getter_matchers)*
                            Ok(::jsi::JsiValue::new_undefined())
                        },
                    }
                }

                fn set(
                    &mut self,
                    name: ::jsi::PropName<'rt>,
                    value: ::jsi::JsiValue<'rt>,
                    rt: &mut ::jsi::RuntimeHandle<'rt>,
                ) -> anyhow::Result<()> {
                  match rt.to_string(&name).as_str() {
                      #(#setter_matchers)*
                      _ => {
                        #(#include_setter_matchers)*

                        ::anyhow::bail!("cannot set {}", rt.to_string(&name))
                    },
                  }
                }

                fn properties(&mut self, rt: &mut ::jsi::RuntimeHandle<'rt>) -> Vec<::jsi::PropName<'rt>> {
                    let props = vec![
                        #(::jsi::PropName::new(#prop_names, rt)),*
                    ];

                    #(#include_prop_names)*

                    props
                }
              }
        }))
    }
}
