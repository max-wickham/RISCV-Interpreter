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

fn deserialize_test_objects<'de, D>(deserializer: D) -> Result<Vec<TestObject>, D::Error>
where
    D: Deserializer<'de>,
{
    let tests: Vec<serde_json::Value> = Deserialize::deserialize(deserializer)?;
    let mut result = Vec::new();
    for test in tests {
        result.push(serde_json::from_value::<TestObject>(test).unwrap());
    }
    Ok(result)
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
    for (index, test) in input.into_iter().enumerate() {
        // let test_name = Ident::new(&format!("test_{}", index), proc_macro2::Span::call_site());
        let test_name = Ident::new(test.name.as_str(), proc_macro2::Span::call_site());
        let struct_name = Ident::new("MyStruct", proc_macro2::Span::call_site());
        // let code = Ident::new(test.code.as_str(), proc_macro2::Span::call_site());
        let code = test.code.as_str();
        tests.push(quote! {
            #[test]
            fn #test_name() {
                let binary: Vec<u8> = assemble(&String::from(#code));
                println!("{:?}",binary);
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
