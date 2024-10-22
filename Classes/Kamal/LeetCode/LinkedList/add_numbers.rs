#[derive(PartialEq)]
struct list_node {
    num: i32,
    next: Option<Box<list_node>>
}

struct mylist {
    head: Option<Box<list_node>>
}

impl mylist {
    fn new() -> mylist {
        println!("New!!!");
        let head = mylist{head:Option::None};
        head
    }

    // self mutable reference
    fn insert_node (&mut self, inum: i32) {
        let new_node = Some(Box::new(
                        list_node {
                        num :inum,
                        next :Option::None}));

        if self.head == Option::None {
            self.head = new_node;
            return;
        }

        let mut curr = &mut self.head.take();

        while curr.unwrap().next != None {
            curr = &mut curr.unwrap().next;
        }
        
        curr.unwrap().next = new_node;
        return;
    }
}

fn main() {
    let mut stu_list = mylist::new();
}
