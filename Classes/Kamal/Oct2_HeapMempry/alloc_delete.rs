struct IdsSensor {
    name: String,
    id: u8
}

impl Drop for IdsSensor {
    fn drop(&mut self) {
        println!("Name : {}", self.name);
        println!("Going to deallocate memory!!!");
    }
}

fn main() {
    let _s1 = Box::new(IdsSensor{name:String::from("ssh modnitor"), id:100});
}