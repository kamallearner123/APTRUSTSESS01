
// fn main() {
//     let mut sec_event = (1020, "This is related SSH key not found".to_string(), 1.1);
//     println!("num = {}, mesg = {}, poss = {}", sec_event.0, sec_event.1, sec_event.2);
//     sec_event.0 = 1030;
//     println!("num = {}, mesg = {}, poss = {}", sec_event.0, sec_event.1, sec_event.2);

//     let msg = &mut sec_event.1;
//     println!("{}", sec_event.1);
//     msg.push_str("..."); // Non-Lexical Lifetime
// }


fn main() {
    let result = {
        let a=10;
        a+1 //return 
    };

    println!("{}", result);
}