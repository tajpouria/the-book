fn main() {
    println!("Hello, world!");

    {
        let foo = String::from("Foo");
        println!("The value of foo is: {}", foo);
    }

    let s1 = String::from("S1");
    let _s1 = borrow(s1);
    println!("In the main the value of s1 is: {}", _s1);

    let s1 = String::from("S1");
    not_borrow(&s1);
    println!("In the main the value of s1 is: {}", s1);

    let mut s1 = String::from("S1");
    mut_not_borrow(&mut s1);
    println!("In the main the value of the s1 is: {}", s1);

    let mut s = String::from("S");

    let r0 = &s;
    let r1 = &s;
    // let r2 = &mut s; error[E0502]: cannot borrow `s` as mutable because it is also borrowed as immutable
    println!("In the main the value of the r0 and r1 are: {}, {}", r0, r1);

    let r2 = &mut s;
    // let r3 = &mut s; error[E0499]: cannot borrow `s` as mutable more than once at a time
    println!("In the main the value of the r2 is: {}", r2);

    let mut s = String::from("Hello World");
    let word = first_word(&s);
    println!("In the main the value of the word is: {}", word);
    s.clear();


    let a = [1;10];
    let slice = &a[..2];
    println!("In the main the value of the slice is: {:?}", slice);
}

fn borrow(s1: String) -> String {
    println!("In the borrow fn the value of s1 is: {}", s1);
    s1
}

fn not_borrow(s: &String) {
    println!("In the not_borrow fn the value of s is: {}", s);
}

fn mut_not_borrow(s: &mut String) {
    s.push_str(", mutated!")
}

fn first_word(s: &str) -> &str {
    let bytes = s.as_bytes();

    for (i, &item) in bytes.iter().enumerate() {
        if item == b' ' {
            return &s[0..i];
        }
    }

    &s[..]
}
