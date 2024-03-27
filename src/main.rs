use std::fmt::{Debug, Display};

fn main() {
    let string1 = String::from("abcd");
    let result;

    {

        let string2 =String::from("xyz");

         result = longest(string1.as_str(), string2.as_str());
    }

        println!("The longest string is {}", result)
}


fn some_function<T, U>(t: T, u : U) -> Result<T, U> 
where 
    T: Display + Clone,
    U: Clone + Debug 
{
    Ok(t)
}

fn longest<'a>(str1 : &'a str, str2: &'a str) -> &'a str {
    if str1.len() > str2.len() {
        str1
    } else {
        str2
    }
}

