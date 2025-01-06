use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, ItemFn, Meta};

#[proc_macro_attribute]
pub fn aoc(attr: TokenStream, item: TokenStream) -> TokenStream {
    let input = parse_macro_input!(item as ItemFn);
    let attrs = attr.to_string();
    
    let part = if attrs.contains("parse") {
        quote! { advent_2024::utils::runner::AocPart::Parse }
    } else if attrs.contains("part1") {
        quote! { advent_2024::utils::runner::AocPart::Part1 }
    } else if attrs.contains("part2") {
        quote! { advent_2024::utils::runner::AocPart::Part2 }
    } else {
        panic!("Invalid AoC part specified");
    };

    let fn_name = &input.sig.ident;
    let fn_block = &input.block;
    let vis = &input.vis;
    let sig = &input.sig;

    quote! {
        #[doc(hidden)]
        #vis #sig {
            #fn_block
        }

        inventory::submit! {
            advent_2024::utils::runner::AocSolution {
                part: #part,
                func: #fn_name,
            }
        }
    }.into()
}
