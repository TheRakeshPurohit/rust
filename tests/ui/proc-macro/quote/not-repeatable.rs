// FIXME(quote): `proc_macro::quote!` doesn't support repetition at the moment, so the stderr is
// expected to be incorrect.
//@ known-bug: #54722

#![feature(proc_macro_quote)]

extern crate proc_macro;

use proc_macro::quote;

struct Ipv4Addr;

fn main() {
    let ip = Ipv4Addr;
    let _ = quote! { $($ip)* };
}
