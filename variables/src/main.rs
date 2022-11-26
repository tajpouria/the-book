fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    const SUBSCRIBER_COUNT: u32 = 100_000;
    println!("The sub count is {}", SUBSCRIBER_COUNT);

    let shadow = "not shadowed";
    let shadow = "shadowed";
    println!("The value of the shadow is: {}", shadow);

    sum = my_function(1, 2)

    if sum < 10 {
    } else if sum < 22 {
    } else {
    }

    let res = if sum < 2 { sum } else { sum + 1 };

    loop {
        println!("once");
        break;
    }

    let mut c = 0;
    let res = loop {
        c += 1;
        if c >= 10 {
            break counter
        }
    }
    println!("Loop has returned: {}", c)

    let num = 10;
    while num != 0 {
        println!("{}", num);
        num -= 1;
    }

    let arr = [1; 10];
    for el in arr.iter(){
        println!("the values is: {}", el);
    }

    for r in 1..10 {
        println!("{}!", r)
    }

    // Single line comment

    /*
     * Multi line comments
     */
}

fn scalar_types(){
    // Integers
    let a: i32 = -1;
    let a: u64 = 1;
    // Floating-point numbers
    let a: f32 = 5.7;
    let a: f64 = 5.7;
    // Booleans
    let a: bool = false;
    // Characters
    let a = 'a';
    println!("The value of a is: {}", a);
}

fn compound_types(){
    let tup = ("hello, world", 100_000);
    let (s, c) = tup;
    let s = tup.0;
    let c = tup.1;

    let arr = [200, 404, 500];
    let not_found = arr[1]

    let bytes = [0; 8];
}

fn my_function(x: i32, y: i32) -> i32 {
    println!("Another function: {}", x);
    println!("Another function: {}", y);
    x + y
}

