#[derive(Debug)]
enum Color {
    Red,
    Green,
}

struct Store {
    inventory: Vec<Color>,
}

impl Store {
    fn get_most_viewed(&self) -> Result<i32, &str> {
        let mut red_count = 0;
        let mut green_count = 0;

        for color in &self.inventory {
            match color {
                Color::Red => red_count += 1,
                Color::Green => green_count += 1,
                _ => return Err("Invalid color"),
            }
        }

        if red_count > green_count {
            Ok(red_count)
        } else {
            Ok(green_count)
        }
    } 
}

fn main() {
    let my_store = Store {
        inventory: vec![Color::Red, Color::Green, Color::Red],
    };

    println!("Store Inventory: {:?}", my_store.inventory);
    println!("Most viewed color: {:?}", my_store.get_most_viewed());
}