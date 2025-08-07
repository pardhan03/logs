use std::{fs, string};
use std::io::Error;

fn string_test(a: String, b: &String, c: &str) {

}

fn main() {

    string_test(String::from("red"), &String::from("red"), "red");

    let text = fs::read_to_string("logs.txt");

    match text {
        Ok(text) => println!("{:#?}",text.len()),
        Err(e) => eprintln!("Failed to read logs.txt: {}", e),
    }

    // let ans = divide(0.5, 0.2);
    // match ans {
    //     Ok(value) => println!("{:#?}",value),
    //     Err(e) => eprintln!("Failed to read logs.txt: {}", e),
    // }

    // let is_email_valid = validate_email(String::from("test@email.com"));
    // match is_email_valid {
    //     Ok(()) => { println!("Email is Valid") },
    //     Err(error) => {eprint!("{:#?}", error)}
    // }
}

// fn divide (a:f64, b:f64) -> Result<f64, Error> {
//     if b== 0.0 {
//         Err(Error::other("Can't divide by 0"))
//     } else {
//         Ok(a/b)
//     }
// }

// fn validate_email(email: String) -> Result<(), Error> {
//     if email.contains("@") {
//         Ok(())
//     } else {
//         Err(Error::other("Email must have an @"))
//     }
// }
