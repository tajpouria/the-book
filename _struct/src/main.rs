struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn main() {
    let user1 = User {
        active: true,
        username: String::from("someone"),
        email: String::from("someone@gmail.com"),
        sign_in_count: 1,
    };

    println!("The value of ther user1 email is: {}", user1.email);

    // user1.email = String::from("newemail@gmail.com"); cannot assign to `user1.email`, as `user1` is not declared as mutable

    let mut user2 = User {
        active: false,
        username: String::from("anotherone"),
        email: String::from("anotheremail@gmail.com"),
        sign_in_count: 2,
    };

    user2.email = String::from("anothernewemail@bing.com");

    println!("The value of the user2 email is: {}", user2.email);


    let user3 = build_user(String::from("yet_another_user"), String::from("yet_another_user_email@mail.org"));

    println!("The value of the sign in count for user3 is: {}", user3.sign_in_count);


    let user4 = User {
        email: String::from("user4_emails@mail.com"),
        ..user3
    };

    println!("The value of the email and username for user3 are: {}, {}", user4.email, user4.username);

}

fn build_user(username: String, email: String) -> User {
    User {
        username,
        email,
        active: true,
        sign_in_count: 0
    }
}
