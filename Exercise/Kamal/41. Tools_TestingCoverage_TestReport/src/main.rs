mod mymod;

fn id<T> (x: T) -> T {
    return x;
}

fn fun() -> u8 {
    return 0;
}
fn main() {
    println!("Hello, world!");
    let x =10;
    println!("Return = {}", id(x));

    let x = "Hello";
    println!("Return = {}", id(x));
}

#[cfg(test)]
mod test {
    use  super::*;
    #[test]
    fn test_01() {
        assert_eq!(fun(), 0);
    }

    #[test]
    fn test_mymod() {
        assert_eq!(mymod::whole_square(1,2), 9);
    }
}

#[cfg(test)]
mod test2 {
    use super::*;
    #[test]
    fn test_01() {
        assert_eq!(fun(), 0);
    }
    
    #[test]
    fn test_mymod() {
        assert_eq!(mymod::whole_square(1,2), 9);
    }

    #[test]
    fn test_main() {
        main();
        assert_eq!(10, id(10));
    }
}
