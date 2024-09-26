
fn add_tail(data:&mut String) {
    data.push_str("...modified by function");
}

fn main() {
    let mut data = String::from("hello");
    let ref_data1:& String = &data;
    let ref_data2:& String = &data;
    let ref_data_write: &mut String = &mut data; //acquired mut ref
    let ref_data3: &String = &data; // ERROR: till 2018
    ref_data_write.push_str(".. adding string");//ERROR
}