struct User {
    username: String,
    email: String,
    sign_in_count: u64,
    active: bool
}


fn main() {
    let mut user = User {
        email: String::from("example@email.com"),
        username: String::from("zebro"),
        sign_in_count: 1,
        active: true
    };

    let a = user.email;

    // struct 更新语法
    let mut user2 = User {
        email: String::from("example@email.com"),
        username: String::from("zebro"),
        sign_in_count: user.sign_in_count
        active: user.active
    };
}

