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
    #[test]
    fn test_case_01(){
        assert_eq!(add(1,2), 43);
    }
}
