fn main() {
    let mut x = 4;
    println!("x is : {}", x);

    {
        let x = 2;
        println!("x is {}", x);
    }

    x = 5;
    println!("x is : {}", x);

    const SECONDS_IN_MINUTES: u32 /*integer*/ = 60;
    print!("Seconds in minutes {}", SECONDS_IN_MINUTES);
}
