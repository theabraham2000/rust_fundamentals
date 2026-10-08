fn lesson_1() {

    // lesson 1: printing things on screen
    // print macro - after printing the content, cursor stays in the same line
    println!("LESSON-1 [PRINTING THINGS ON SCREEN]");
    print!("5"); 
    print!("+"); 
    print!("3 = "); 
    print!("8");
    print!("\n"); // Manually writing the newline
    println!("============");

}

fn lesson_2() {

    println!("LESSON-2 [COMMENTS]");
    // lesson 2: writing comments
    // single line: use //
    // multi line: use /* */
    /* Rust
        is 
       Cool
    */
    //println macro - after printing the content cursor moves to the next line
    println!("abraham_mathews was here!");
    println!();
    println!("============");

}

fn lesson_3() {

    println!("LESSON-3 [VARIABLES]");
    // lesson 3: variables (use let)
    let name: &str = "abraham_mathews";
    let handle: &str= "theabraham2000";
    let age: i32 = 25;
    let kpop_fan: bool = true;

    println!("My name is {}. Follow me on X: @{}. I am {} years old.", name, handle, age);
    println!("Kpop Fan? {}.", kpop_fan);
    println!();
    // Notes:
    // By default the variables are immutable and its values cannot be updated
    // To make it mutable use mut keyword like:
    let mut stars: i32 = 54023;
    println!("number of stars in an hypothetical galaxy is {}.", stars);
    stars = 89483;
    println!("number of stars in an hypothetical galaxy is {}.", stars);
    println!("============");
}

fn lesson_4() {
    println!("LESSON-4 [CONSTANTS]");
    const BIRTHDAY: &str = "25 OCT 2026";
    println!("My birthday is on {BIRTHDAY}.");
    println!("============");
}

fn lesson_5() {
    println!("LESSON-5 [Shadowing]");
    let kpop_bias: &str = "Gaeul";
    {
        let kpop_bias: &str = "Ryujin";
        println!("{kpop_bias}");
    }
    println!("{kpop_bias}");
    println!("============");
}

fn lesson_6() {
    println!("LESSON-6 [Compound Datatypes: Tuples]");

    let gaeul_details: (&str, i32, f64) = ("Gaeul", 24, 7813.13);
    let name: &str = gaeul_details.0;
    let age: i32 = gaeul_details.1;
    let randm_no: f64 = gaeul_details.2;

    println!("Name = {name}");
    println!("Age = {age}");
    println!("Random Number = {randm_no}");
    println!("============");
}

fn lesson_7() {
    println!("LESSON-7 [Compound Datatypes: Arrays]");

    let kpop_entertainment_companies: [&str; 4] = ["SM", "YG", "JYP", "Starship"];

    let first = kpop_entertainment_companies[0];
    let third = kpop_entertainment_companies[2];
    println!("{first}");
    println!("{third}");

    println!("============");
}

fn lesson_8(title: &str) {
    println!("LESSON-8 [{title}]");

    fn kpop_star_details(name: &str, group_name: &str, age: i32) {
        println!("Name: {name} \nGroup: {group_name} \nAge: {age}");
    }
    
    kpop_star_details("Yujin", "IVE", 23);
    println!("============");
}

fn lesson_9(title: &str) {
    println!("LESSON-9 [{title}]");
    fn dob() -> String {
        "25 OCT 2025".to_string()
    }
    let output_dob = dob();
    println!("DOB: {output_dob}");

    println!("============");
}

fn lesson_10(title: &str) {
    println!("LESSON-10 [{title}]");

    //Control Flow using if ... else ...
    let rooftop_filled: bool = false;
    if rooftop_filled == true {
        println!("Kindly go to the balcony, rooftop is filled!")
    } else {
        println!("Welcome, you can go to the rooftop!")
    }

    //Using if in a let statement
    let password: &str = "63135";
    let login_status: &str = if password == "64284" {"Login Successful"} else {"Login Failed"};
    println!("{login_status}");

    println!("============");
}

fn lesson_11(title: &str) {
    println!("LESSON-11 [{title}]");

    //Looping
    let mut count: i32 = 0;
    loop {
        if count < 10 {
            println!("{count}");
            count = count + 1;
        } else {
            break;
        }
    }

    println!("============");
}

fn main() {

    println!("============");
    lesson_1();
    lesson_2();
    lesson_3();
    lesson_4();
    lesson_5();
    lesson_6();
    lesson_7();
    lesson_8("Functions");
    lesson_9("Statements and Expressions");
    lesson_10("Control Flow");
    lesson_11("Looping");

}
