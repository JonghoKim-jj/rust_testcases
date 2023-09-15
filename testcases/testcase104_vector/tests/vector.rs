#[test]
fn vector() {
    let _vec = vec![1, 2, 3];   // immutable vector can not append/push its member
    let mut mutable_vec: std::vec::Vec<i32> = vec![1, 2, 3];

    mutable_vec.push(4);

    let immutable_vec = mutable_vec;   // You can pass ownership of mutable vector to immutable vector
    let _third = &immutable_vec[2];

    match immutable_vec.get(2) {
        Some(_third) => println!("Exist: {}", _third),
        None => println!("Not Exist")
    }


    match immutable_vec.get(100) {
        Some(_) => println!("Exist"),
        None => println!("Not Exist")
    }
}

#[test]
#[should_panic]
fn accessing_via() {
    let immutable_vec = vec![1, 2, 3, 4];
    immutable_vec[100];   // Should PANIC
}
