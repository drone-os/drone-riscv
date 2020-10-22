use drone_macros_core::parse_ident;
use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::TokenStream as TokenStream2;
use quote::{format_ident, quote};
use syn::{
    braced, parenthesized,
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Attribute, ExprPath, Ident, LitInt, Token, Visibility,
};

struct Input {
    thr: Thr,
    local: Local,
    index: Index,
    init: Init,
    threads: Threads,
}

struct Thr {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
    tokens: TokenStream2,
}

struct Local {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
    tokens: TokenStream2,
}

struct Index {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
}

struct Init {
    attrs: Vec<Attribute>,
    vis: Visibility,
    ident: Ident,
}

struct Threads {
    threads: Vec<Thread>,
}

enum Thread {
    Exception(ThreadSpec),
    Timer(ThreadSpec),
    External(u32, ThreadSpec),
    Software(ThreadSpec),
}

struct ThreadSpec {
    attrs: Vec<Attribute>,
    vis: Visibility,
    kind: ThreadKind,
    ident: Ident,
}

enum ThreadKind {
    Inner,
    Outer(ExprPath),
    Naked(ExprPath),
}

impl Parse for Input {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let thr = input.parse()?;
        input.parse::<Token![;]>()?;
        let local = input.parse()?;
        input.parse::<Token![;]>()?;
        let index = input.parse()?;
        input.parse::<Token![;]>()?;
        let init = input.parse()?;
        input.parse::<Token![;]>()?;
        let threads = input.parse()?;
        if !input.is_empty() {
            input.parse::<Token![;]>()?;
        }
        Ok(Self { thr, local, index, init, threads })
    }
}

impl Parse for Thr {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        parse_ident!(input, "thread");
        input.parse::<Token![=>]>()?;
        let vis = input.parse()?;
        let ident = input.parse()?;
        let input2;
        braced!(input2 in input);
        let tokens = input2.parse()?;
        Ok(Self { attrs, vis, ident, tokens })
    }
}

impl Parse for Local {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        parse_ident!(input, "local");
        input.parse::<Token![=>]>()?;
        let vis = input.parse()?;
        let ident = input.parse()?;
        let input2;
        braced!(input2 in input);
        let tokens = input2.parse()?;
        Ok(Self { attrs, vis, ident, tokens })
    }
}

impl Parse for Index {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        parse_ident!(input, "index");
        input.parse::<Token![=>]>()?;
        let vis = input.parse()?;
        let ident = input.parse()?;
        Ok(Self { attrs, vis, ident })
    }
}

impl Parse for Init {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let attrs = input.call(Attribute::parse_outer)?;
        parse_ident!(input, "init");
        input.parse::<Token![=>]>()?;
        let vis = input.parse()?;
        let ident = input.parse()?;
        Ok(Self { attrs, vis, ident })
    }
}

impl Parse for Threads {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        parse_ident!(input, "threads");
        input.parse::<Token![=>]>()?;
        let input2;
        braced!(input2 in input);
        let mut threads = Vec::new();
        while !input2.is_empty() {
            let attrs = input2.call(Attribute::parse_outer)?;
            let ident = input2.parse::<Ident>()?;
            input2.parse::<Token![=>]>()?;
            if attrs.is_empty() && ident == "external" {
                let input3;
                braced!(input3 in input2);
                while !input3.is_empty() {
                    let attrs = input3.call(Attribute::parse_outer)?;
                    let num = input3.parse::<LitInt>()?.base10_parse()?;
                    input3.parse::<Token![:]>()?;
                    let vis = input3.parse()?;
                    let kind = input3.parse()?;
                    let ident = input3.parse()?;
                    threads.push(Thread::External(num, ThreadSpec { attrs, vis, kind, ident }));
                    if !input3.is_empty() {
                        input3.parse::<Token![,]>()?;
                    }
                }
            } else if attrs.is_empty() && ident == "software" {
                let input3;
                braced!(input3 in input2);
                while !input3.is_empty() {
                    let attrs = input3.call(Attribute::parse_outer)?;
                    let vis = input3.parse()?;
                    let kind = input3.parse()?;
                    let ident = input3.parse()?;
                    threads.push(Thread::Software(ThreadSpec { attrs, vis, kind, ident }));
                    if !input3.is_empty() {
                        input3.parse::<Token![,]>()?;
                    }
                }
            } else if ident == "exception" {
                let vis = input2.parse()?;
                let kind = input2.parse()?;
                let ident = input2.parse()?;
                threads.push(Thread::Exception(ThreadSpec { attrs, vis, kind, ident }));
            } else if ident == "timer" {
                let vis = input2.parse()?;
                let kind = input2.parse()?;
                let ident = input2.parse()?;
                threads.push(Thread::Timer(ThreadSpec { attrs, vis, kind, ident }));
            } else {
                return Err(input2.error(format!("Unexpected ident `{}`", ident)));
            }
            if !input2.is_empty() {
                input2.parse::<Token![,]>()?;
            }
        }
        Ok(Self { threads })
    }
}

impl Parse for ThreadKind {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        match input.fork().parse::<Ident>() {
            Ok(ident) if ident == "outer" => {
                input.parse::<Ident>()?;
                let input2;
                parenthesized!(input2 in input);
                let path = input2.parse()?;
                Ok(Self::Outer(path))
            }
            Ok(ident) if ident == "naked" => {
                input.parse::<Ident>()?;
                let input2;
                parenthesized!(input2 in input);
                let path = input2.parse()?;
                Ok(Self::Naked(path))
            }
            _ => Ok(Self::Inner),
        }
    }
}

pub fn proc_macro(input: TokenStream) -> TokenStream {
    let Input { thr, local, index, init, threads } = parse_macro_input!(input as Input);
    let Threads { threads } = threads;
    let threads = enumerate_threads(threads);
    let def_array = def_array(&thr, &threads);
    let def_index = def_index(&index, &thr, &threads);
    let def_init = def_init(&init, &index, &thr, &threads);
    let def_core_thr = def_core_thr(&thr, &local);
    let expanded = quote! {
        mod __thr {
            #[allow(unused_imports)]
            use super::*;
            #def_array
            #def_index
            #def_init
            #def_core_thr
        }
        #[allow(unused_imports)]
        pub use self::__thr::*;
        ::drone_riscv::reg::assert_taken!("plic_hart0_m_claim_complete");
    };
    expanded.into()
}

fn enumerate_threads(threads: Vec<Thread>) -> Vec<(Option<usize>, Thread)> {
    let mut counter = 0;
    threads
        .into_iter()
        .map(|thread| match &thread {
            Thread::Exception(spec)
            | Thread::Timer(spec)
            | Thread::External(_, spec)
            | Thread::Software(spec) => {
                let ThreadSpec { kind, .. } = spec;
                match kind {
                    ThreadKind::Inner | ThreadKind::Outer(_) => {
                        let num = counter;
                        counter += 1;
                        (Some(num), thread)
                    }
                    ThreadKind::Naked(_) => (None, thread),
                }
            }
        })
        .collect()
}

fn def_array(thr: &Thr, threads: &[(Option<usize>, Thread)]) -> TokenStream2 {
    let Thr { ident: thr_ident, .. } = thr;
    let mut array_tokens = vec![quote! {
        #thr_ident::new(0)
    }];
    for (num, _) in threads {
        if let Some(num) = num {
            array_tokens.push(quote! {
                #thr_ident::new(#num)
            });
        }
    }
    let array_len = array_tokens.len();
    quote! {
        static mut THREADS: [#thr_ident; #array_len] = [#(#array_tokens),*];
    }
}

fn def_index(index: &Index, thr: &Thr, threads: &[(Option<usize>, Thread)]) -> TokenStream2 {
    let Index { attrs: index_attrs, vis: index_vis, ident: index_ident } = index;
    let Thr { ident: thr_ident, .. } = thr;
    let mut tokens = Vec::new();
    let mut index_tokens = Vec::new();
    let mut index_ctor_tokens = Vec::new();
    for (num, thread) in threads {
        match thread {
            Thread::Exception(spec)
            | Thread::Timer(spec)
            | Thread::External(_, spec)
            | Thread::Software(spec) => {
                let ThreadSpec { attrs, vis, kind, ident } = spec;
                match kind {
                    ThreadKind::Inner | ThreadKind::Outer(_) => {
                        let field_ident = format_ident!("{}", ident.to_string().to_snake_case());
                        let struct_ident = format_ident!("{}", ident.to_string().to_pascal_case());
                        tokens.push(quote! {
                            #(#attrs)*
                            #[derive(Clone, Copy)]
                            #vis struct #struct_ident {
                                __priv: (),
                            }

                            unsafe impl ::drone_core::token::Token for #struct_ident {
                                #[inline]
                                unsafe fn take() -> Self {
                                    #struct_ident {
                                        __priv: (),
                                    }
                                }
                            }

                            unsafe impl ::drone_core::thr::ThrToken for #struct_ident {
                                type Thr = #thr_ident;

                                const THR_NUM: usize = #num;
                            }
                        });
                        index_tokens.push(quote! {
                            #(#attrs)*
                            #vis #field_ident: #struct_ident,
                        });
                        index_ctor_tokens.push(quote! {
                            #field_ident: ::drone_core::token::Token::take(),
                        });
                    }
                    ThreadKind::Naked(_) => {}
                }
            }
        }
    }
    quote! {
        /// Reset thread token.
        #[derive(Clone, Copy)]
        pub struct Reset {
            __priv: (),
        }

        unsafe impl ::drone_core::token::Token for Reset {
            #[inline]
            unsafe fn take() -> Self {
                Reset {
                    __priv: (),
                }
            }
        }

        unsafe impl ::drone_core::thr::ThrToken for Reset {
            type Thr = #thr_ident;

            const THR_NUM: usize = 0;
        }

        #(#tokens)*

        #(#index_attrs)*
        #index_vis struct #index_ident {
            /// Reset thread token.
            pub reset: Reset,
            #(#index_tokens)*
        }

        unsafe impl ::drone_core::token::Token for #index_ident {
            #[inline]
            unsafe fn take() -> Self {
                Self {
                    reset: ::drone_core::token::Token::take(),
                    #(#index_ctor_tokens)*
                }
            }
        }

        unsafe impl ::drone_riscv::thr::ThrTokens for #index_ident {}
    }
}

fn def_init(
    init: &Init,
    index: &Index,
    thr: &Thr,
    threads: &[(Option<usize>, Thread)],
) -> TokenStream2 {
    let Init { attrs: init_attrs, vis: init_vis, ident: init_ident } = init;
    let Index { ident: index_ident, .. } = index;
    let Thr { ident: thr_ident, .. } = thr;
    let mut handlers = Vec::new();
    let mut exception = quote!(None);
    let mut timer = quote!(None);
    let mut external = Vec::new();
    for (num, thread) in threads {
        match thread {
            Thread::Exception(ThreadSpec { kind, .. }) => {
                let (handler, path) = def_handler(thr, num, kind);
                handlers.push(handler);
                exception = quote!(Some(#path));
            }
            Thread::Timer(ThreadSpec { kind, .. }) => {
                let (handler, path) = def_handler(thr, num, kind);
                handlers.push(handler);
                timer = quote!(Some(#path));
            }
            Thread::External(source, ThreadSpec { kind, .. }) => {
                let source = *source as usize;
                let (handler, path) = def_handler(thr, num, kind);
                handlers.push(handler);
                if external.len() < source {
                    external.resize(source, quote!(None));
                }
                external[source - 1] = quote!(Some(#path));
            }
            Thread::Software(_) => {}
        }
    }
    quote! {
        #(#init_attrs)*
        #init_vis struct #init_ident {
            __priv: (),
        }

        unsafe impl ::drone_core::token::Token for #init_ident {
            #[inline]
            unsafe fn take() -> Self {
                Self {
                    __priv: (),
                }
            }
        }

        unsafe impl ::drone_riscv::thr::ThrsInitToken for #init_ident {
            type ThrTokens = #index_ident;

            type Thread = #thr_ident;

            const EXCEPTION_HANDLER: Option<unsafe extern "C" fn()> = #exception;

            const TIMER_INTERRUPT_HANDLER: Option<unsafe extern "C" fn()> = #timer;

            const EXTERNAL_INTERRUPT_HANDLERS: &'static [Option<unsafe extern "C" fn()>] = &[
                #(#external),*
            ];
        }

        #(#handlers)*
    }
}

fn def_core_thr(thr: &Thr, local: &Local) -> TokenStream2 {
    let Thr { attrs: thr_attrs, vis: thr_vis, ident: thr_ident, tokens: thr_tokens } = thr;
    let Local { attrs: local_attrs, vis: local_vis, ident: local_ident, tokens: local_tokens } =
        local;
    quote! {
        ::drone_core::thr! {
            array => THREADS;

            #(#thr_attrs)*
            thread => #thr_vis #thr_ident { #thr_tokens };

            #(#local_attrs)*
            local => #local_vis #local_ident { #local_tokens };
        }
    }
}

fn def_handler(thr: &Thr, num: &Option<usize>, kind: &ThreadKind) -> (TokenStream2, TokenStream2) {
    let Thr { ident: thr_ident, .. } = thr;
    match kind {
        ThreadKind::Inner => {
            let ident = format_ident!("thr_handler_inner_{}", num.unwrap());
            (
                quote! {
                    unsafe extern "C" fn #ident() {
                        unsafe {
                            ::drone_riscv::thr::thread_resume::<#thr_ident>(#num);
                        }
                    }
                },
                quote!(#ident),
            )
        }
        ThreadKind::Outer(path) => {
            let ident = format_ident!("thr_handler_outer_{}", num.unwrap());
            (
                quote! {
                    unsafe extern "C" fn #ident() {
                        unsafe {
                            ::drone_riscv::thr::thread_call::<#thr_ident>(#num, #path);
                        }
                    }
                },
                quote!(#ident),
            )
        }
        ThreadKind::Naked(path) => (quote!(), quote!(#path)),
    }
}
