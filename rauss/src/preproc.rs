// use crate::types::*;


// pub fn pre_proc_direct(directives: &Vec<Directive>, source_code: &mut Vec<u8>) {
//     let mut is_macro = false;
//     let mut macro_founded = false;
//     let mut beg_row: usize = 0;
//     let mut beg_col: usize = 0;
//     let mut length:  usize = 0;
//     let mut macro_str = String::new();

//     let mut column: usize = 0;
//     let mut row: usize = 1;

//     for sym_code in source_code.clone() {
//         column += 1;
//         if sym_code == 0xA {
//             if column == 1 {
//                 column = 0;
//                 row += 1;
//                 continue;
//             }
//             column = 0;
//             row += 1;
//         }

//         let symbol: char = sym_code as char;

//         if symbol == ' ' {
//             continue;
//         }

//         if column != 1 && symbol == '!' {
//             is_macro = true;
//             beg_col = column.clone();
//             beg_row = row.clone();
//             continue;
//         }

//         if is_macro {
//             match symbol {
//                 'A'..='Z' | '_' => macro_str.push(symbol),
//                 _ => {
//                     is_macro = false;
//                     if is_macro_dir(directives, macro_str.clone()) {
//                         length = macro_str.len();
//                         proc(beg_col, beg_row, length, get_macro(directives, macro_str.clone()), source_code);
//                         macro_founded = false;
//                     }
//                     macro_str = String::new();
//                 }
//             }

//         }


//     }
// }

// fn proc(b_col: usize, b_row: usize, length: usize, repl_str: String, code: &mut Vec<u8>) {
//     let mut column: usize = 0;
//     let mut row: usize = 1;

//     let mut found = false;
//     let mut len = length.clone();

//     let mut start_index: usize = 0;

//     for (index, sym_code) in code.iter().enumerate() {
//         column += 1;
//         if *sym_code == 0xA {
//             if column == 1 {
//                 column = 0;
//                 row += 1;
//                 continue;
//             }
//             column = 0;
//             row += 1;
//         }

//         if row == b_row && column == b_col {
//             found = true;
//         }

//         if found && len == 0 {
//             found = false;
//         }
//         if found && len != 0 {
//             if len == length {
//                 start_index = index;
//             }
//             len -= 1;
//         }
//     }
//     for _ in 0..=length {
//         code.remove(start_index.clone());
//     }
//     let info_ir = repl_str.as_bytes();
//     let mut info: Vec<u8> = Vec::new();
//     for i in 1..=info_ir.len() {
//         info.push(info_ir[info_ir.len()-i]);
//     }
//     for i in info { 
//         code.insert(start_index.clone(), i);
//     }
// }

// fn is_macro_dir(directives: &Vec<Directive>, macro_str: String) -> bool {
//     let mut result = false;
//     for directive in directives {
//         match directive {
//             _ => ()
//         }
//     }
//     result
// }

// fn get_macro(directives: &Vec<Directive>, macro_str: String) -> String {
//     let mut result = String::new();
//     for directive in directives {
//         match directive {
//             _ => ()
//         }
//     }
//     result
// }

