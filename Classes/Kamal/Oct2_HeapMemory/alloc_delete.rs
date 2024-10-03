#[derive(Debug, Clone)]
struct IdsSensor {
    name: String,//4bytes
    id: u8,//1 byte
    next: Option<Box<IdsSensor>>
}

impl Drop for IdsSensor {
    fn drop(&mut self) {
        println!("Name : {}", self.name);
        println!("Going to deallocate memory!!!");
    }
}

// pool = 100 * sizeof(strucut student);

// Node *head = (Node *)malloc(sizeof(Node));
// Node *pool = (Node*)malloc(1000000*sizeof(Node));

fn main_1() {
    // Ownership
    let mut s1: Box<IdsSensor> = Box::new(IdsSensor{name:String::from("ssh monitor"), id:100, next:None});
    let mut s2: Box<IdsSensor> = Box::new(IdsSensor{name:String::from("telnet monitor"), id:200, next:None});
    s1.next = Some(s2);
}

fn main() {
    main_1();
    println!("Exiting from main function");
}