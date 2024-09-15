fn main() {
    let nums:[i32;7]= [1,2,3,4,1,2,3]; // 1. Programmer can not give more or less size than required.
    println!("nums = {:?}", nums);
    
    let mut result = nums[0];
    for item in 1..=nums.len() { // 2. If you try to access more than size, it crashes (Undefined behaviour in C/C++)
        result = result^nums[item];
    }
    
    println!("result = {}", result);
}
