enum MyOption <T, T2> {
    Some(T),
    None,
    ERR(T2),
}

#[derive(Debug)]
struct Point {
    x: i32,
    y: i32,
}

fn func() ->  MyOption<Point, String> {
    //MyOption::Some(Point{x:10, y:20})
    //MyOption::None
    MyOption::ERR(String::from("ERROS message"))
}

fn main() {
    let a = {
        while let MyOption::ERR(result) = func() { 
            println!("got some value {:?}", result);
            break;
        }
        // } else { 
        //     //println!("Else")
        //     -1
        // }
    };

    println!("a = {:?}", a);
}