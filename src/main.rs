mod calc;
mod generator;
mod json;

fn main() {
    // let p = Path::new("output/output");
    // let sum = generator::generate(p, 1_000_000);
    // println!("Expected sum: {}", sum.unwrap());
    // match untyped_example() {
    //     Ok(_) => (),
    //     Err(e) => print!("Error {:?}", e)
    // }
    let mut b: i32 = 3;
    test_mut_copy(&mut b);
    println!("New b: {:?}", b);
}


fn test_mut_copy(b: &mut i32) -> i32 {
    *b += 5;
    *b
}
// fn untyped_example() -> Result<()> {
//     // Some JSON input data as a &str. Maybe this comes from the user.
//     let data = r#"
//         {
//             "name": "\u000AJohn Doe",
//             "age": 43e3,
//             "phones": [
//                 "+44 1234567",
//                 "+44 2345678"
//             ]
//         }"#;

//     // Parse the string of data into serde_json::Value.
//     let v: Value = serde_json::from_str(data)?;

//     // Access parts of the data by indexing with square brackets.
//     println!("Please call {} at the number {}", v["name"], v["phones"][0]);
//     println!("Age {}", v["age"]);

//     Ok(())
// }
