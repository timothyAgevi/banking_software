mod account;
use std::io;

fn update(){}
fn delete(){}

fn transfer(){}
fn quit(){}
//register function
fn register() -> account::Account{
    println!("\nWelcome to the registration page.\nPlease enter your name to create an account:");
    let mut name= String::new();
    io::stdin().read_line(&mut name).expect("msg");
    let user: account::Account = account::Account::new(&name);
    println!("\tCreated user {}\tAccount balance: 0",name);
    user
}

//function deposit
fn deposit(user: account::Account) -> f32{
    println!("\nEnter amount you wish to deposit:");
    let mut amount= String::new();
    io::stdin().read_line(&mut amount).expect("msg");
    let amount = amount.trim().parse::<f32>().expect("invalid input");
    let balance: f32 = user.getBalance(); 
    println!("\tSuccesfuly deposited {}$\n\tNew account balance {}$",amount,balance+&amount);
    balance+&amount
}
//withdraw function
fn withdraw(user: account::Account) -> f32{
    //TODO: Code to identify user and respective balance

    println!("\nEnter amount you wish to withdraw:");
    let mut amount= String::new();
    io::stdin().read_line(&mut amount).expect("msg");
    let amount = amount.trim().parse::<f32>().expect("invalid input");
    
    let balance: f32 = user.getBalance(); 
    if balance-amount>0.0 {
        println!("\tSuccesfuly withdrew {}$\n\tNew account balance {}$",amount,balance-&amount);
    }else {
        println!("\tAvailable amount {}$, insufficient account balance",balance);
    }
    balance-&amount
}