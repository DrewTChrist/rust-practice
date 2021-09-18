fn main() {
    let number: i32 = 3;
    let numtwo: i32 = 6;

    // if else
    if number < 5 {
        println!("true");
    } else {
        println!("false");
    }

    // if else and else if
    if numtwo % 4 == 0 {
        println!("numtwo is divisible by 4");
    } else if numtwo % 3 == 0 {
        println!("numtwo is divisible by 3");
    } else if numtwo % 2 == 0 {
        println!("numtwo is divisible by 2");
    } else {
        println!("numtwo is not divisible by 4, 3 or 2");
    }

    // if in a let statement
    let condition = true;
    let numthree = if condition { 5 } else { 6 };
    let numfour = if condition { 5 + 1 } else { 6 };
    // Both results of branch must be of the same type
    // let numfive = if condition { "six" } else { 6 };
    let numfive = if condition { "six" } else { "three" };
    println!("numthree: {}", numthree);
    println!("numfour: {}", numfour);
    println!("numfive: {}", numfive);


    // This will loop forever
    // loop {
    //     println!("again!");
    // }
    
    let mut counter: i32 = 0;

    let result: i32 = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    println!("The result is {}", result);


    // conditional loops with while
    let mut countertwo: i32 =  3;
    while countertwo != 0 {
        println!("{}!", countertwo);
        countertwo -= 1;
    }

    // looping through a collection with while
    let arr = [10, 20, 30, 40, 50];
    let mut index = 0;

    while index < 5 {
        println!("arr[index]: {}", arr[index]);
        index += 1;
    }

    // looping through a collection with for
    for element in arr.iter() {
        println!("element: {}", element);
    }

    // for loop with an index
    for i in (1..4).rev() {
        // rev() reverses a range
        println!("{}!", i);
    }

}
