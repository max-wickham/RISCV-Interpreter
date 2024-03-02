extern crate toast_interpreter;

struct YourJsonStruct {
    // Define the structure of your JSON data here
    field1: i32,
    field2: i32,
}

#[cfg(test)]
mod arithmetic_int_tests {
    use serde::Deserialize;
    use serde_json::Result;
    use std::error::Error;
    use std::fs;
    use std::path::Path;
    use toast_interpreter::assembler::assemble;
    use toast_interpreter::interpret;
    use toast_interpreter::CPUState;
    #[test]
    fn test_files() {
        let input_folder_path = Path::new("test_gen/binary_files");
        let output_folder_path = Path::new("test_gen/result_files");

        for entry in fs::read_dir(input_folder_path).unwrap() {
            let entry = entry;
            let file_path = entry.unwrap().path();
            // Check if the file has a .s extension
            if let Some(ext) = file_path.extension() {
                if ext == "bin" {
                    // Build the corresponding JSON file path
                    let json_file_name = file_path.file_stem().unwrap().to_string_lossy() + ".json";
                    let json_file_path = output_folder_path.join(json_file_name.as_ref());
                    println!("{:?}", json_file_path);
                    // Read the JSON file content
                    let json_content = fs::read_to_string(&json_file_path);
                    println!("{:?}", json_content);
                    // Parse the JSON content into your struct
                    let parsed_json: Vec<i32> =
                        serde_json::from_str(&json_content.unwrap()).unwrap();

                    // Now you can use parsed_json as needed
                    println!("{:?}", parsed_json);

                    let mut cpu_state = CPUState::new();
                    let result = interpret(file_path.to_string_lossy().as_ref(), &mut cpu_state);
                    if result.is_err() {
                        panic!("Error: {}", result.err().unwrap());
                    }
                    println!("{:?}", file_path.to_string_lossy().as_ref());
                    assert_eq!(
                        cpu_state.registers[parsed_json[0] as usize] as u32,
                        parsed_json[1] as u32
                    );
                }
            }
        }

        // use std::env;
        // if let Ok(current_dir) = env::current_dir() {
        //     println!("Current working directory: {}", current_dir.display());
        // } else {
        //     eprintln!("Failed to get the current working directory");
        // }
        // let mut cpu_state = CPUState::new();

        // let result = interpret("test_gen/binary_files/add.bin",  &mut cpu_state);
        // if result.is_err() {
        //     panic!("Error: {}", result.err().unwrap());
        // }
        // println!("Register 0: {}", cpu_state.registers[3]);
        // assert_eq!(cpu_state.registers[3], 12);
        // let instruction_1 =
        // assert_eq!(2 + 2, 4);
    }

    // Add more tests as needed
}
