const _MAX_POINTS: u32 = 100_000;
/* Comment */

fn main() {
    let mut x = 5;
    println!("The value of x is: {}", x);
    x = 6;
    println!("The value of x is: {}", x);

    let _a = 3;
    let _a = _a + 1; // shadow variable

    let _f: bool = false;

    let _i: u128 = 1000000;
    let _ii: f64 = 1.0;

    let _c: char = 'ñ';

    let _tup: (i32, i32, char) = (500, 100, 'a');
    let (_j, _k, _l) = _tup; // unpack tuple
    let _last = _tup.2; // indexing

    // Arrays have fixed length, all values must have same type
    let _arr = [1, 2, 3, 4];
    let _last = _arr[3]; // indexing

    let _s = {
        let y = 1;
        y + 1
    };


    x = some_function(x);
    println!("{}", x);

    let number = 3;

    if number < 5 { // condition must be bool
        println!("condition was true");
    } else {
        println!("condition was false");
    }

    let _number2 = if number > 0 { 5 } else { 6 };

    let mut counter = 0;

    let _result = loop {
        counter += 1;

        if counter == 10 {
            break counter * 2;
        }
    };

    let mut number = 10;
    while number != 0 {
        println!("{}!", number);
        number -= 1;
    }

    let a = [10, 20, 30, 40, 50];

    for element in a.iter() {
        println!("the value is: {}", element);
    }
}

fn some_function(i: i32) -> i32 {
    println!("param: {}", i);
    i + 1
}
