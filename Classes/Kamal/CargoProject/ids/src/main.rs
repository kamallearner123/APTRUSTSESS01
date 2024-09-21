/// # Add function
/// - It adds two floating point numbers
/// - Returns f32
/// 
/// 
#[allow(dead_code)]
fn add (a:i32,b:i32) ->i32 {
    a+b
}
fn main() {
    println!("Hello, world!");
    let _flag_1: i32 = 10;

}

#[cfg(test)]
mod testsuite01 {
    use super::*;
    // #[test]
    // fn test_case_01(){
    //     assert_eq!(add(1,2), 43);
    // }

    #[test]
    fn test_case_02(){
        assert_eq!(add(1,2), 3);
    }

    #[test]
    fn Testcase_case_03(){
        assert_eq!(add(1,2), 3);
    }
}
