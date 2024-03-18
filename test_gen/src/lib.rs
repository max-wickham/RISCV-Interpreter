use proc_macro::TokenStream;
use quote::quote;
use syn::{Ident, parse_macro_input, LitStr};
use serde::{Serialize, Deserialize, Deserializer};

#[derive(Serialize, Deserialize, Debug)]
struct TestObject {
    name: String,
    code: String,
    result: (usize, i64),
}

#[proc_macro]
pub fn generate_tests(input: TokenStream) -> TokenStream {
    let file_name_lit = parse_macro_input!(input as LitStr);
    let file_name = file_name_lit.value();

    // Read the content of the specified JSON file
    let input_str = match std::fs::read_to_string(&file_name) {
        Ok(content) => content,
        Err(err) => {
            // Handle file reading error here
            panic!("Failed to read file {}: {:?}", file_name, err);
        }
    };

    let input: Vec<TestObject> = match serde_json::from_str(&input_str) {
        Ok(obj) => obj,
        Err(err) => {
            // Handle parsing error here
            panic!("Failed to parse input from file {}: {:?}", file_name, err);
        }
    };

    let mut tests = Vec::new();
    for (_, test) in input.into_iter().enumerate() {
        let test_name = Ident::new(test.name.as_str(), proc_macro2::Span::call_site());
        let code = test.code.as_str();
        let result_register = test.result.0;
        let result = test.result.1;
        tests.push(quote! {
            #[test]
            fn #test_name() {
                let binary: Vec<u8> = assemble(&String::from(#code));
                println!("{:?}",binary);
                let mut cpu_state = CPUState::new();
                interpret_max_cycles(&binary, &mut cpu_state, 20);
                assert_eq!(
                    cpu_state.registers[#result_register as usize] as u32,
                    #result as u32
                );
            }
        });
    }

    // Combine all generated tests into a single TokenStream
    let expanded = quote! {
        #[derive(Debug)]
        struct MyStruct {
            field1: String,
            field2: String,
            field3: String,
        }

        #(#tests)*
    };

    TokenStream::from(expanded)
}
