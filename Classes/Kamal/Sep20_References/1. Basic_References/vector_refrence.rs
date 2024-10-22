fn main() {
    let mut v: Vec<i32> = vec![1, 2, 3];
    let mut v1: Vec<&mut i32> = Vec::new();

    for i in &mut v {
        // 1. Getting reference of each element of v and storing in v1.
        // 2. Since v1 is having reference of v, we can't use v after this point.
        v1.push(i); 
    }

    *v1[0] = 5; // 3. Changing the value of v1[0] will change the value of v[0] as well.
    let b = *v1[0];  // 4. Getting the value of v1[0] will get the value of v[0] as well.
    let a = v[0]; // 5. Note: As per old verions: This should give error as v is borrowed by v1.
                  // But as per Non-Lexical Lifetime, this will work as v is not used after v1.push(i).
                  // Values are copied to a, b. (not moved)
                  // After this point v1 is not accessible.

    println!("{} {}",a,b);
}

//
