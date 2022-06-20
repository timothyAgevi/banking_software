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