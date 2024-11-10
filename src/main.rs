
mod even; 
mod  fib;
mod return_string;
mod struct_example;
mod rectangle;
mod enum_example;
mod inbuilt_enum;
mod result_enum;
mod vector_example;
mod hashmap_example;
mod filter_and_mao;
mod generic_example;
mod trait_example;
mod lifetime_example;


use std::collections::HashMap;
use chrono::{Local,Utc};
fn main() {
    println!("{}",even::is_even(21));
    println!("{}",fib::fib_series(5));
    let my_string = String::from("Hello World");
    let length = return_string::get_string_length(&my_string);
    println!("The number of character in string is: {}",length);

    // struct example
    let user = struct_example::User{
        name: String::from("prakash lahagun"),
        age: 25,
        email: String::from("prakash@gmail.com"),
    };

    println!("{}",user.name);
    println!("{}",user.age);
    println!("{}",user.email);


    // Rectangle Area

    let rect = rectangle::Rectangle{
        width:10,
        height:10
    };
    println!("{}",rect.area());

    //Enum Example
    let rect_area = enum_example::Shape::Rectangle(1.0,2.0);
    let circle_area = enum_example::Shape::Circle(2.0);
    let result1 = enum_example::calculate_area(rect_area);
    let result2 = enum_example::calculate_area(circle_area);
    println!("The area of rectangle {}",result1);
    println!("The area of circle {}",result2);

    //Enum Second Example
    let index = inbuilt_enum::find_first_a(String::from("prakash"));
    match index  {
        Some(value)=> println!("index is {}", value),
        None => println!("a not found"),
    };

    let file_path = String::from("/home/prakash/Test/rust_journey/src/a.txt");
    match result_enum::read_from_file_a(file_path)  {
        Ok(content) => println!("File content {}", content),
        Err(e) => eprint!("Error reading file: {}",e),
    }

    // UTC Package
    let now = Utc::now();
    println!("Current Date {}",now);
    let format_date = now.format("%Y-%M-%D %H:%M:%S");
    println!("Formatted Date: {}",format_date);

    let local_time = Local::now();
    println!("Local Time {}",local_time);


    //Collections Vector example
    let mut numbers = Vec::new();
    numbers.push(1);
    numbers.push(2);
    numbers.push(3);

    //print first number
    println!("Frist number is {}",numbers[0]);

    numbers.pop();

    for i in &numbers{
        println!("The number is {}",i);
    }

    println!("The number  are {:?}",numbers);


    //Vector filter even example
    let mut vect = Vec::new();
    vect.push(1);
    vect.push(2);
    vect.push(3);

    println!("The even numbers are {:?}",vector_example::even_filter(vect));


    // HashMap Example  
    let mut users :HashMap<String,i32> =  HashMap::new();
    users.insert(String::from("PK"), 21);
    users.insert(String::from("DK"), 24);

    let user1 = users.get("PK");
    println!("Age is {}",user1.unwrap());

    let second_user_age = users.get("DK");

    match second_user_age {
        Some(age) => println!("Age is {}",age),
        None => println!("User not found")
    }

    // Second Example on HashMap
    let user_vect = vec![(String::from("Ramesh"),22),(String::from("Hori"),28)];
    let hm = hashmap_example::group_by_user(user_vect);
    println!("{:?}",hm);

    //Iter example
    let smthg :Vec<i32> = vec![1,2,3,4,5];
    let answer = filter_and_mao::filter_map_example(smthg);
    println!("{:?}",answer);

    // Generics Example

    let generic_result1 =  generic_example::largest_num(10,20);
    let generic_result2 =  generic_example::largest_num('a','b');

    println!("Large number is {}",generic_result1);
    println!("Large character is {}",generic_result2);

    // Trait example
    let article = trait_example::Article{
        headline: String::from("Kathmandu Flood"),
        content: String::from("1000 dead and 2000 people lost there livehood.")
    };
    trait_example::notify(&article);

    //Lifetime example 
    let first_string = String::from("Prakash");
    let second_string = String::from("Shyam");
    let ans = lifetime_example::largest_number(&first_string, &second_string);
    println!("The large string is {}",ans);
}
