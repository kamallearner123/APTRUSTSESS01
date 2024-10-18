fn longest(x:&str, y:&str) -> String {
    if x.len() > y.len() {
        x.to_string()
    } else {
        y.to_string()
    }
}
// fn longest<'a> (x:&'a str, y:&'a str) -> &'a str {
//     if x.len() > y.len() {
//         x
//     } else {
//         y
//     }
// }

fn main() {
    println!("Longset string : {:?}", longest("kamal123", "Sandy"));
}