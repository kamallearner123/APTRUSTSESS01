#[derive(Debug)]
struct Artist {
    skills: Vec<String>,
    name:String
}

fn print_skills(artists :&mut Vec<Artist>) {
    for artist in artists {
        artist.name.push_str("...");
        for skill in &artist.skills {
            println!("Skill: {}", skill);
        }
    }
}

fn main() {
    let mut artists = Vec::new();
    let david = Artist{name:String::from("David"), 
                                skills:vec![String::from("dance")]};
    artists.push(david);

    let john = Artist{name:String::from("John"), 
                              skills:vec![String::from("sing")]};
    artists.push(john);

    print_skills(&mut artists);

    println!("Length of the vector = {}", artists.len());

    // let nums = vec![1,2,3];
    // println!("length = {}", nums.len());
}