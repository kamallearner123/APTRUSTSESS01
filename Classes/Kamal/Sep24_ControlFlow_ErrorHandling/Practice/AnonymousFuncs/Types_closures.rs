// fn main() {
//     let add = |a, b| a + b;
//     let add2 = |a, b| a + b;
//     let result = add(2, 3);  // Using the closure
//     let result2 = add2(2.1, 3.1); 
//     println!("2 + 3 = {}", result);
//     println!("2.1 + 3.1 = {}", result2);
// }


fn <T:Fn(f64)>make_fp() {
    let tax_perc = 0.3; //30%
    let calculate_tax = |salary| println!("tax = {}", salary*tax_perc);
    return calculate_tax;
}

fn main() {
    let fp = make_fp();
    fp(100000.0);
}


// fn cal_anonymous <T:Fn(f64)>(func:T) {
//     func(10000.1);
// }

// fn main() {
//     let tax_perc = 0.3; //30%
//     let calculate_tax = |salary:f64| println!("tax = {}", salary*tax_perc);
//     calculate_tax(100000 as f64);
//     cal_anonymous(calculate_tax);
// }


// fn main() {
//     let calculate_square = |num: i32| -> i32 {
//         num * num
//     };

//     let result = calculate_square(5);  // Closure returns the square of 5
//     println!("Square of 5 is: {}", result);
// }