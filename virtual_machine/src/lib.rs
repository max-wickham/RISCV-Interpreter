extern crate proc_macro;

use proc_macro::TokenStream;
use proc_macro2::{TokenStream as TokenStream2, TokenTree};
use quote::quote;
use syn::parse_macro_input;

#[proc_macro]
pub fn vm(input: TokenStream) -> TokenStream {
    // Convert the input TokenStream into a TokenStream2
    let tokens = parse_macro_input!(input as proc_macro2::TokenStream);
    // let input: TokenStream2 = input.into();
    // Iterate through each token
    // for token in tokens {
    //     // Here you can manipulate or process each token as needed
    //     // For example, printing each token
    //     println!("{}", token);
    //     match token {
    //         TokenTree::Ident(ident) if ident.to_string() == "range" => {
    //             // Token is "range", do something
    //             println!("Encountered 'range' token");
    //         }
    //         _ => {
    //             // Token is not "range", do something else
    //             // For example, printing each token
    //             println!("{}", token);
    //         }
    //     }
    // }

    // Vector to store tokens in range
    let mut range_tokens = Vec::new();
    let mut in_range_mode = false;
    // Iterate through each token
    for token in tokens {
        if in_range_mode {
            // If we're in range mode, we collect tokens until we find ";"
            if let TokenTree::Punct(punct) = &token {
                if punct.to_string() == ";" {
                    // We've reached the end of the range, exit range mode
                    in_range_mode = false;
                } else {
                    // Collect the token in range
                    range_tokens.push(token);
                }
            } else {
                // Collect the token in range
                range_tokens.push(token);
            }
        } else {
            // Check if the token is "range"
            if let TokenTree::Ident(ident) = &token {
                if ident.to_string() == "range" {
                    // We've encountered the "range" token, enter range mode
                    in_range_mode = true;
                }
            }
            // For simplicity, just print tokens outside the range mode
            println!("{}", token);
        }
    }

    // Echo back the input tokens
    // input.into(s)

    let output = quote! {
        #&tokens
    };
    // Convert the `proc_macro2::TokenStream` back to `proc_macro::TokenStream`
    output.into()
}
