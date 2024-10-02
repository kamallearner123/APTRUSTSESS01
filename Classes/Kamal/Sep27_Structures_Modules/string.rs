#[derive(Debug)]
struct MyString {
    len: usize,
    cap: usize,
    name: String,
}

impl MyString {
    // define methods for MyString
    fn from(data:String) -> MyString {
        let Instance =  MyString{len:data.len(), cap:100};
        if Instance.len > Instance.cap {
            println!("Exceeding the data");
        }
        return Instance;
    }

    fn push_str(&mut self, data: &String) {
        //self.name.push_str(data);
        self.len = self.len+data.len();
    }

    fn print_details(self) {
        println!("{:?}", self);
    }
}

fn main() {
    // let s2 = String::from("hello world");
    // String is structure
    // from is method of String
    //let s1 = MyString{len:0, cap:0, buffer:None};

    let mut s2: MyString = MyString::from(String::from("Hello"));
    //let s3 = s2;
    let s1: MyString = MyString{len:10, .. s2};//
    println!("{:?}", s1);
    println!("{:?}", s2);
}