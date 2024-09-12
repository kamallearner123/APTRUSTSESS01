
mod mymod;
mod vegs;

const PI:f32 = 3.141;

///
/// # History
/// areaCircle: Function which calculates area of circle and returns
/// - r : Radius in f32
/// - Returns: Area in f32
/// ## References
/// - Please refer [`whole_square`](mymod::whole_square)
fn areaCircle(r:f32) -> f32 {
    return  PI*r.powf(2.0);
}

/// Decription about main function
fn main() {
    //! 
    //! At least 3 arguments are required.
    //! 
    let radius:f32 = (10.0);
    let my_vec:Vec<i32> = vec![1,2,3,4];

    println!("Radius = {}", radius);

    /*
     * Calling areaCircle function. 
    */
    println!("Area = {}",areaCircle(radius));

}
