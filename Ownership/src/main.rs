//Ownership
//1data->>>1owner
fn foo() {
    //create a vec!
    //gives Ownership of the vec objcet to v
    let mut v = vec![1, 2, 3, 4];

    v.pop();
    v.push(5);
    // At the end of the scope, v1 goes out of scope.
    // v1 still owns the Vec object, so it can be cleaned up.
    //
    //
    // Move sentiments
    let v1 = v;
    // Ownership of the vec objcet  v moves to v1
    println!("{:?}", v); //error:-used of move value v

    //Borrowing
    let bs = vec![1, 2, 3, 4];
    //Borrow immutability
    for b in bs {
        println!("{}", b)
    }
    //Borrow mutability
    for b in &mut bs {
        *b = *b + 1;
    }
}
