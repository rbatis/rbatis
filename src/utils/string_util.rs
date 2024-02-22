// /// convert name to snake name
// pub fn to_snake_name(name: &str) -> String {
//     let len = name.len();
//     let bytes = name.as_bytes();
//     let mut new_name = String::with_capacity(name.len());
//     let mut index = 0;
//     for x in bytes {
//         let c = *x as char;
//         if c.is_ascii_uppercase() {
//             if index != 0 && (index + 1) != len {
//                 new_name.push('_');
//             }
//             new_name.push(c.to_ascii_lowercase() as char);
//         } else {
//             new_name.push(c);
//         }
//         index += 1;
//     }
//     return new_name;
// }
