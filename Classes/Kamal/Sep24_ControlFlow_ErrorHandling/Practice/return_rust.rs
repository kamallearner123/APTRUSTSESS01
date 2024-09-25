fn fun() -> i32 {
	println!("Not returning values");
	0
}

fn main() {
	let Result = fun();
	println!("Result = {}", Result);
}
