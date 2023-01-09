fn main() {
    let mut user1 = User {
        email: String::from("someones@example.com"),
        username: String::from("someonename123"),
        active: true,
        sign_in_count: 1,
    };
    user1.email = String::from("anotheremail@example.com");

    // option to copy from another struct
    // let user2 = User {
    //     active: user1.active,
    //     username: user1.username,
    //     email: String::from("yetanotheremail@exaple.com"),
    //     sign_in_count: user1.sign_in_count,
    // };
    // but this is better
    let user2 = User {
        email: String::from("canwestoptheemails@example.com"),
        ..user1
    };
}

struct User {
    active: bool,
    username: String,
    email: String,
    sign_in_count: u64,
}

fn build_user(email: String, username: String) -> User {
    User {
        active: true,
        username,
        email,
        sign_in_count: 1,
    }
}
// Tuple struct without named fields to create different types
struct Color(i32, i32, i32);
struct Point(i32, i32, i32);

// Structs without any field
struct AlwaysEqual;
