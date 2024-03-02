// use itertools::concat;
// use std::collections::HashMap;

// fn assemble(assembly: String) -> Vec<u8> {
//     assembly.replace(":\n", ":");
//     assembly.replace(":\r\n", ":");
//     let lines: Vec<&str> = assembly.lines().collect();
//     let lines: Vec<String> = lines.iter()
//     .map(|&s| s
//         .split('#')
//         .next()
//         .unwrap_or_default()
//         .replace("\t", " ")
//         .replace(" ", ",")
//     )
//     .collect();
//     let lines: Vec<Vec<String>> = lines.iter().map(|line| line
//         .split_whitespace()
//         .filter(|&word| !word.is_empty())
//         .map(String::from).collect()).collect();

//     let lines: Vec<&Vec<String>> = lines.iter().filter(|line| line.len() > 0).collect();
//     let mut labels: HashMap<String, i64> = HashMap::new();
//     let concat_lines: Vec<&String> = lines.clone().into_iter().flatten().collect();
//     for (index, line) in concat_lines.iter().enumerate(){
//         if line.contains(":") {
//             labels.insert(
//                 line.split(":").next().unwrap_or_default().to_string(),
//                 index as i64 * 4);
//         }
//     }
//     let mut condensed_lines : Vec<Vec<String>> = vec![];
//     let mut index = 0;
//     let len_lines = lines.len();
//     while index < len_lines{
//         let mut line: Vec<String> = lines[index].clone();
//         let next_line:Vec<String> = lines[index+1].clone();
//         if line[0].contains(":") && (line.len() == 1) && (index != lines.len()){
//             line.extend(next_line);
//             condensed_lines.push(line);
//             index += 1;
//         } else {
//             condensed_lines.push(line);
//         }
//         index += 1;
//     }

//     // filter out labels
//     let lines: Vec<Vec<String>> = lines.iter().map(|line|
//         line.iter().filter(|word| !word.contains(":")).map(String::from).collect()
//     ).collect();



//     vec![0, 0, 0]
// }
