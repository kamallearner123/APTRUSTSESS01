
fn main() {
    let nums = vec![4,2,1,0,2,3,5,6,7,-10];
    let mut search = 1;
    let mut out = nums.len();
    nums.sort();
    let result = loop{
        for i in 0..nums.len() {
            println!("search = {}, nums[i] = {}", search, nums[i]);
            if search == nums[i] {
                out = i;
                break;
            }
        };

        if out < nums.len() {
            search = search + 1;
            out = nums.len();
        } else {
            break search;
        }
        
    };

    println!("num = {}", result);
}