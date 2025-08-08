use std::{fs, string};
use std::io::Error;

fn extract_error(text: &str) -> Vec<String> {
    let split_text = text.split("\n");

    let mut results = vec![];

    for line in split_text {
        if line.starts_with("ERROR") {
            results.push(line.to_string());
        }
    }
    results
}

fn string_test(a: String, b: &String, c: &str) {

}

fn main() {

    string_test(String::from("red"), &String::from("red"), "red");

    // let mut error_logs = vec![];

    let text = fs::read_to_string("logs.txt").expect("Failed to read logs.txt");
    let error_logs =  extract_error(&text);
    fs::write("error.txt", error_logs.join("\n")).expect("Failed to write error.txt");
    // match text {
    //     Ok(text) => {
    //         let error_logs = extract_error(text.as_str());
    //         // println!("{:#?}", error_logs);
    //         match fs::write("error.txt", error_logs.join("\n")) {
    //             Ok(()) => {},
    //             Err(e) => { eprintln!("{:#?}", e)}
    //         }
    //     },
    //     Err(e) => {
    //         eprintln!("Failed to read logs.txt: {}", e)
    //     },
    // }

    // at this point text will drop from the stack and now error_logs will not point any value
    // this will cause dangling pointer erro
    // println!("{:#?}", error_logs)

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
