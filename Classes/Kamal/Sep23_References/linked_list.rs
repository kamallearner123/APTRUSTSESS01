#[derive(Debug)]
struct node {
    data:i32,
    next:Option<Box<node>>
}

fn main() {
    let mut head = node{data:10, next:None};
    println!("{:?}", head);

    let mut next = Box::new(node{data:200, next:None});
    head.next = Some(next);
    println!("{:?}", head.next);

    let back = &head.next;

    let mut next = Box::new(node{data:200, next:None});
    head.next = Some(next);
    println!("{:?}", head.next);
}