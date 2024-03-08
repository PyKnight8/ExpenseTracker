//Expense Tracker in Rust
use std::fs::{self,OpenOptions};
use std::io::{stdin, Write};
use std::process::exit;


fn log_expense(expn:String,price:String){
    println!("Expense : {}Price : {}\n",expn,price);

    //Time calculation
    let curr_time = chrono::Local::now();
    let formatted_time = curr_time.format("%d/%m/%Y").to_string();
    let final_log = format!("Expense : {}Price : {}Date: {}\n",expn,price,formatted_time);
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("Expenses.txt")
        .unwrap();
    if let Err(e) = writeln!(file,"{}", final_log) {
        println!("Couldn't Write to file! : {}",e)
    }

}

fn enter_expenses(){
    println!("Enter the expense>");
    let mut expense_name = String::new();
    stdin().read_line(&mut expense_name).expect("Failed to read expense ");
    let mut price = String::new();
    println!("Enter the Price>");
    stdin().read_line(&mut price).unwrap();
    //let parsed_price:f64 = price.trim().parse().expect("Error Parsing String "); // can be used later for calculations
    log_expense(expense_name, price);
}

fn print_expenses(){
    if fs::metadata("Expenses.txt").is_ok(){
    let result = fs::read_to_string("Expenses.txt").unwrap();
    println!("{:?}",result);
    }
    else{
        println!("Couldn't Find Expenses.txt , initialized file...");
        let result = fs::write("Expenses.txt", "Initialized\n");
        println!("{:?}",result);
    }
}
fn main(){
    loop {
        println!("Welcome to ExpenseTracker!\nChoose one of the following options:\n1-Log Expense\n2-Print Expenses\n3-Exit");
        let mut choice = String::new();
        std::io::stdin().read_line(&mut choice).expect("Failed to read your input!");
        let choice_in:String = choice.trim().parse().unwrap();
        if choice_in == "1"{
            enter_expenses()
        }
        else if choice_in =="2"{
            print_expenses()
        }
        else if choice_in =="3"{
            exit(0);
        }
    }
}