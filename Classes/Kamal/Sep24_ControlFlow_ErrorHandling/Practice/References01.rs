// // # References and scope
// fn main() {
//     let mut a = 10;
//     let fun: (a) = {
//         let ra = &mut a;
//         *ra = 20;
//         println!("{ra}");
//     }
//     println!("{a}");
// }


// # Multiple level mutable referece
fn main() {
    let mut a = 10;
    {
        let ra = &a;
        println!("ra = {}", ra);
        a = 100;
    }

    println!("a = {}, ra = {}", a, ra);
}



// // #Multiple level immutable references
// fn main() {
//     let mut a = 10;
//     let ra = & a;
//     let rra = &ra;
//     println!("{},{},{}",a,*ra,**rra);
// }

// Reference to struct
// Nno need of *
// struct data {
//     id:i32,
//     name :String
// }
// fn main() {
//     let mut s1 = data{name:String::from("Kamal"), id:10};
//     let mut rs1 = &mut s1;
//     println!("rst1.id = {}", rs1.id);
//     let rrs1 = &mut rs1;
//     rrs1.id = 101;
//     println!("rrs1.id = {}", rrs1.id);
// }

