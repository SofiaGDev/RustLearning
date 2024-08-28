fn main() {
    println!("Data Types");

    let _x: i32 = 2; //i32 integer, it's using 32 bits
                    /*types
                    types of integer in bits
                    i8
                    i16
                    i32
                    i64
                    i128 */

    let _x: u32 = 247; //u means that can't hold numbers with signs ex: -35

    let _flating_point: f32 = 10.5;

    let _true_or_false: bool = true;
 
    let _letter: char = 'y';


    //tupples

    let mut tup: (i32, bool, i32, i32) = (1, true, -36, 35);

    tup.0= 8;

    println! ("{}", tup.0 );

    // arrays

    let _arr:[i32; 4] = [5, 3, 6, 3];

    
}
