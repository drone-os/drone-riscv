use inflector::Inflector;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenStream as TokenStream2};
use quote::{format_ident, quote};
use syn::{
    braced,
    parse::{Parse, ParseStream, Result},
    parse_macro_input, Attribute, Ident, LitInt, Token, Visibility,
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
    ident: Ident,
}

impl Parse for Input {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let mut thr = None;
        let mut local = None;
        let mut index = None;
        let mut init = None;
        let mut threads = None;
        while !input.is_empty() {
            let attrs = input.call(Attribute::parse_outer)?;
            let ident = input.parse::<Ident>()?;
            input.parse::<Token![=>]>()?;
            if ident == "thread" {
                if thr.is_none() {
                    thr = Some(Thr::parse(input, attrs)?);
                } else {
                    return Err(input.error("multiple `thread` specifications"));
                }
            } else if ident == "local" {
                if local.is_none() {
                    local = Some(Local::parse(input, attrs)?);
                } else {
                    return Err(input.error("multiple `local` specifications"));
                }
            } else if ident == "index" {
                if index.is_none() {
                    index = Some(Index::parse(input, attrs)?);
                } else {
                    return Err(input.error("multiple `index` specifications"));
                }
            } else if ident == "init" {
                if init.is_none() {
                    init = Some(Init::parse(input, attrs)?);
                } else {
                    return Err(input.error("multiple `init` specifications"));
                }
            } else if attrs.is_empty() && ident == "threads" {
                if threads.is_none() {
                    threads = Some(input.parse()?);
                } else {
                    return Err(input.error("multiple `threads` specifications"));
                }
            }
            if !input.is_empty() {
                input.parse::<Token![;]>()?;
            }
        }
        Ok(Self {
            thr: thr.ok_or_else(|| input.error("missing `thread` specification"))?,
            local: local.ok_or_else(|| input.error("missing `local` specification"))?,
            index: index.ok_or_else(|| input.error("missing `index` specification"))?,
            init: init.ok_or_else(|| input.error("missing `init` specification"))?,
            threads: threads.ok_or_else(|| input.error("missing `threads` specification"))?,
        })
    }
}

impl Thr {
    fn parse(input: ParseStream<'_>, attrs: Vec<Attribute>) -> Result<Self> {
        let vis = input.parse()?;
        let ident = input.parse()?;
        let input2;
        braced!(input2 in input);
        let tokens = input2.parse()?;
        Ok(Self { attrs, vis, ident, tokens })
    }
}

impl Local {
    fn parse(input: ParseStream<'_>, attrs: Vec<Attribute>) -> Result<Self> {
        let vis = input.parse()?;
        let ident = input.parse()?;
        let input2;
        braced!(input2 in input);
        let tokens = input2.parse()?;
        Ok(Self { attrs, vis, ident, tokens })
    }
}

impl Index {
    fn parse(input: ParseStream<'_>, attrs: Vec<Attribute>) -> Result<Self> {
        let vis = input.parse()?;
        let ident = input.parse()?;
        Ok(Self { attrs, vis, ident })
    }
}

impl Init {
    fn parse(input: ParseStream<'_>, attrs: Vec<Attribute>) -> Result<Self> {
        let vis = input.parse()?;
        let ident = input.parse()?;
        Ok(Self { attrs, vis, ident })
    }
}

impl Parse for Threads {
    fn parse(input: ParseStream<'_>) -> Result<Self> {
        let input2;
        braced!(input2 in input);
        let mut exception = false;
        let mut timer = false;
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
                    let idx = input3.parse::<LitInt>()?.base10_parse()?;
                    input3.parse::<Token![:]>()?;
                    let vis = input3.parse()?;
                    let ident = input3.parse()?;
                    threads.push(Thread::External(idx, ThreadSpec { attrs, vis, ident }));
                    if !input3.is_empty() {
                        input3.parse::<Token![;]>()?;
                    }
                }
            } else if attrs.is_empty() && ident == "software" {
                let input3;
                braced!(input3 in input2);
                while !input3.is_empty() {
                    let attrs = input3.call(Attribute::parse_outer)?;
                    let vis = input3.parse()?;
                    let ident = input3.parse()?;
                    threads.push(Thread::Software(ThreadSpec { attrs, vis, ident }));
                    if !input3.is_empty() {
                        input3.parse::<Token![;]>()?;
                    }
                }
            } else if ident == "exception" {
                let vis = input2.parse()?;
                let ident = input2.parse()?;
                if exception {
                    return Err(input2.error("multiple `exception` threads"));
                }
                threads.push(Thread::Exception(ThreadSpec { attrs, vis, ident }));
                exception = true;
            } else if ident == "timer" {
                let vis = input2.parse()?;
                let ident = input2.parse()?;
                if timer {
                    return Err(input2.error("multiple `timer` threads"));
                }
                threads.push(Thread::Timer(ThreadSpec { attrs, vis, ident }));
                timer = true;
            } else {
                return Err(input2.error(format!("Unexpected ident `{}`", ident)));
            }
            if !input2.is_empty() {
                input2.parse::<Token![;]>()?;
            }
        }
        Ok(Self { threads })
    }
}

pub fn proc_macro(input: TokenStream) -> TokenStream {
    let Input { thr, local, index, init, threads } = parse_macro_input!(input as Input);
    let Threads { threads } = threads;
    let def_thr_pool = def_thr_pool(&thr, &local, &index, &threads);
    let def_init = def_init(&thr, &index, &init, &threads);
    let expanded = quote! {
        #def_thr_pool
        #def_init
        ::drone_riscv::reg::assert_taken!("plic_hart0_m_claim_complete");
    };
    expanded.into()
}

#[allow(clippy::cast_possible_truncation)]
fn def_init(thr: &Thr, index: &Index, init: &Init, threads: &[Thread]) -> TokenStream2 {
    let Thr { ident: thr_ident, .. } = thr;
    let Init { attrs: init_attrs, vis: init_vis, ident: init_ident } = init;
    let Index { ident: index_ident, .. } = index;
    let mut exception = LitInt::new("0_u16", Span::call_site());
    let mut timer = LitInt::new("0_u16", Span::call_site());
    let mut external = Vec::new();
    for (idx, thread) in threads.iter().enumerate() {
        let idx = LitInt::new(&format!("{}_u16", idx + 1), Span::call_site());
        match thread {
            Thread::Exception(_) => {
                exception = idx;
            }
            Thread::Timer(_) => {
                timer = idx;
            }
            Thread::External(num, _) => {
                let num = *num as usize;
                if external.len() < num {
                    external.resize(num, LitInt::new("0_u16", Span::call_site()));
                }
                external[num - 1] = idx;
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
            type Thread = #thr_ident;

            type ThrTokens = #index_ident;

            const EXCEPTION_HANDLER: u16 = #exception;

            const TIMER_HANDLER: u16 = #timer;

            const EXTERNAL_HANDLERS: &'static [u16] = &[
                #(#external),*
            ];
        }
    }
}

fn def_thr_pool(thr: &Thr, local: &Local, index: &Index, threads: &[Thread]) -> TokenStream2 {
    let Thr { attrs: thr_attrs, vis: thr_vis, ident: thr_ident, tokens: thr_tokens } = thr;
    let Local { attrs: local_attrs, vis: local_vis, ident: local_ident, tokens: local_tokens } =
        local;
    let Index { attrs: index_attrs, vis: index_vis, ident: index_ident } = index;
    let plic_num = format_ident!("{}_plic_num", thr_ident.to_string().to_snake_case());
    let resume = format_ident!("{}_resume", thr_ident.to_string().to_snake_case());
    let mut threads_tokens = Vec::new();
    let mut plic_num_tokens = Vec::new();
    for (idx, thread) in threads.iter().enumerate() {
        match thread {
            Thread::Exception(spec)
            | Thread::Timer(spec)
            | Thread::External(_, spec)
            | Thread::Software(spec) => {
                let ThreadSpec { attrs, vis, ident, .. } = spec;
                threads_tokens.push(quote! {
                    #(#attrs)* #vis #ident
                });
                if let Thread::External(num, _) = thread {
                    let idx = LitInt::new(&format!("{}_u16", idx), Span::call_site());
                    plic_num_tokens.push(quote! {
                        #idx => ::core::num::NonZeroU32::new(#num)
                    });
                }
            }
        }
    }
    quote! {
        ::drone_core::thr::soft! {
            #(#thr_attrs)*
            thread => #thr_vis #thr_ident {
                plic_num: ::core::option::Option<::core::num::NonZeroU32> = #plic_num(index);
                #thr_tokens
            };

            #(#local_attrs)*
            local => #local_vis #local_ident {
                #local_tokens
            };

            #(#index_attrs)*
            index => #index_vis #index_ident;

            threads => {
                #(#threads_tokens;)*
            };

            resume => #resume;
        }

        const fn #plic_num(index: u16) -> ::core::option::Option<::core::num::NonZeroU32> {
            match index {
                #(#plic_num_tokens,)*
                _ => None,
            }
        }

        #[inline]
        unsafe fn #resume(thr: &#thr_ident) {
            ::core::mem::drop(::drone_core::thr::Thread::fib_chain(thr).drain());
            if let Some(num) = thr.plic_num {
                use ::drone_core::reg::prelude::*;
                use ::drone_core::token::Token;
                use ::drone_riscv::map::reg::plic;
                let plic_claim_complete = plic::Hart0MClaimComplete::<Srt>::take();
                plic_claim_complete.store(|r| r.write_claim_complete(num.get()));
            }
        }
    }
}
