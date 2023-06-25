use axum::{response::Html};
use minijinja::render;
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::fs::{File, OpenOptions};
use std::io::{Read, Write};
use std::time::{Instant};

use crate::html::{
    DEPOSIT, FAILURE, HISTORY, HOME, LOGIN, LOGINACTIVITY, REMOVESUCCESS, SIGNUP,
    SIGNUPFAILURE, SIGNUPSUCCESS, SUCCESS, USERPAGE, WITHDRAW, ASSIGN, ASSIGNSUCCESS,LOGINFAILURE
};


fn read(accno: u32) -> User {
    let start = Instant::now();

    let user_path = format!("user-{}.json", accno);

    let mut file = File::open(user_path).unwrap();
    let mut contents = String::new();
    file.read_to_string(&mut contents).unwrap();

    let user: User = serde_json::from_str(&contents).unwrap();

    let duration = start.elapsed();
    println!("reading a user's file-time elapsed: {:?}", duration);
    user
}

fn write(user: User, accno: u32) {
    let start = Instant::now();
    let user_path = format!("user-{}.json", accno);

    let updated_contents = serde_json::to_string_pretty(&user).unwrap();

    let mut updated_file = OpenOptions::new()
        .write(true)
        .truncate(true)
        .open(user_path)
        .unwrap();
    updated_file.write_all(updated_contents.as_bytes()).unwrap();

    let duration = start.elapsed();
    println!("updating a user's file-time elapsed: {:?}", duration);
}

fn new_file(user: User, accno: u32) {
    let start = Instant::now();

    let user_path = format!("user-{}.json", accno);
    let json = serde_json::to_string(&user).unwrap();
    let mut file = File::create(user_path).expect("Failed to create file");
    file.write_all(json.as_bytes())
        .expect("Failed to write to file");

    let duration = start.elapsed();
    println!("creating a new file for user-time elapsed: {:?}", duration);
}

fn file_exists(filename: &str) -> bool {
    let start = Instant::now();
    let current_dir = env::current_dir().expect("Failed to get current directory");

    let entries = fs::read_dir(current_dir).expect("Failed to read directory");

    for entry in entries {
        if let Ok(entry) = entry {
            let file_name = entry.file_name();
            if let Some(name) = file_name.to_str() {
                if name == filename {
                    let duration = start.elapsed();
                    println!("checking if a file with similar name exists(true condition)-time elapsed: {:?}", duration);
                    return true;
                }
            }
        }
    }
    let duration = start.elapsed();
    println!("checking if a file with similar name exists(false condition)-time elapsed: {:?}", duration);
    false
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Transaction {
    pub amount: f64,
    pub date: String,
    pub category: String,
    pub crdr: String,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct SpendingCategories {
    pub bills: f64,
    pub food: f64,
    pub vacation: f64,
    pub misc: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct Assigned {
    pub bills_assigned: f64,
    pub food_assigned: f64,
    pub vacation_assigned: f64,
    pub misc_assigned: f64,
}

#[derive(Debug, Deserialize, Serialize, Clone)]
pub struct User {
    pub acc_no: u32,
    pub name: String,
    pub balance: f64,
    pub assigned_balance: f64,
    pub history: Vec<Transaction>,
    pub spending: SpendingCategories,
    pub assigned: Assigned,
}

pub async fn home() -> Html<String> {
    let start = Instant::now();

    let r = render!(HOME);

    let duration = start.elapsed();
    println!("rendering HOME-time elapsed: {:?}", duration);

    Html(r)
}

pub async fn signup() -> Html<String> {
    let start = Instant::now();

    let n = render!(SIGNUP);

    let duration = start.elapsed();
    println!("rendering SIGNUP-time elapsed: {:?}", duration);

    Html(n)
}

#[derive(Deserialize)]
pub struct SignupInput {
    id: u32,
    name: String,
}

pub async fn signupactivity(form: axum::Form<SignupInput>) -> Html<String> {
    let start = Instant::now();
    let accno = form.id;
    let accname = &form.name;
    println!("{}, {}", accno, accname);

    let user_path = format!("user-{}.json", accno);

    let filename = user_path;

    let new_user = User {
        acc_no: accno,
        name: accname.to_string(),
        balance: 0.0,
        assigned_balance: 0.0,
        history: [].to_vec(),
        spending: SpendingCategories { bills: (0.0), food: (0.0), vacation: (0.0), misc: (0.0) },
        assigned: Assigned { bills_assigned: 0.0, food_assigned: 0.0, vacation_assigned: (0.0), misc_assigned: 0.0 }
    };

    if file_exists(&filename) {
        let b = render!(SIGNUPFAILURE);
        return Html(b);
    }

    new_file(new_user, accno);

    let u = render!(SIGNUPSUCCESS);

    let duration = start.elapsed();
    println!("SIGNUP activity-time elapsed: {:?}", duration);

    Html(u)
}

pub async fn login() -> Html<String> {
    let start = Instant::now();

    let r = render!(LOGIN);

    let duration = start.elapsed();
    println!("rendering LOGIN-time elapsed: {:?}", duration);

    Html(r)
}

#[derive(Deserialize)]
pub struct LoginInput {
    id: u32,
}

pub async fn loginactivity(form: axum::Form<LoginInput>) -> Html<String>{
    let start = Instant::now();

    let accno = form.id;

    let user_path = format!("user-{}.json", accno);

    let filename = user_path;

    if file_exists(&filename) {
        let r = render!(LOGINACTIVITY, userid => form.id);

        let duration = start.elapsed();
        println!("LOGIN activity(success)-time elapsed: {:?}", duration);

        return Html(r);
    }

    let b = render!(LOGINFAILURE, userid => form.id);

    let duration = start.elapsed();
    println!("LOGIN activity(failure)-time elapsed: {:?}", duration);

    Html(b)
}

pub async fn userpage(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();

    let active_user = params.to_string();
    let accno:u32 = active_user.parse().unwrap();

    let user = read(accno);
    let r = render!(USERPAGE, user => user);

    let duration = start.elapsed();
    println!("rendering USERPAGE-time elapsed: {:?}", duration);

    Html(r)
}

pub async fn history(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();

    let active_user = params.to_string();
    let accno:u32 = active_user.parse().unwrap();

    let user = read(accno);
    let r = render!(HISTORY, user => user);

    let duration = start.elapsed();
    println!("rendering HISTORY-time elapsed: {:?}", duration);

    Html(r)
}

pub async fn deposit(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();
    let active_user = params.to_string();
    let accno:u32 = active_user.parse().unwrap();

    let user = read(accno);
    let q = render!(DEPOSIT, user => user);

    let duration = start.elapsed();
    println!("rendering DEPOSIT-time elapsed: {:?}", duration);

    Html(q)
}

#[derive(Deserialize)]
pub struct UserInput {
    id: u32,
    amount: f64,
    category: String,
}

pub async fn depositactivity(form: axum::Form<UserInput>) -> Html<String> {
    let start = Instant::now();

    let accno = form.id;
    let amount = form.amount;
    let ctgry = &form.category;
    let current_time = chrono::offset::Local::now().to_string();

    let mut user = read(accno);

    user.balance += amount;
    user.assigned_balance += amount;
    user.history.insert(
        0,
        Transaction {
            amount,
            date: current_time,
            category: ctgry.to_string(),
            crdr: "cr".to_string()
        },
    );

    let y = render!(SUCCESS, user => user);
    write(user, accno);

    let duration = start.elapsed();
    println!("DEPOSIT activity-time elapsed: {:?}", duration);

    Html(y)
}

pub async fn withdraw(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();

    let active_user = params.to_string();
    let accno:u32 = active_user.parse().unwrap();

    let user = read(accno);
    let s = render!(WITHDRAW, user => user);

    let duration = start.elapsed();
    println!("rendering WITHDRAW-time elapsed: {:?}", duration);

    Html(s)
}

pub async fn withdrawactivity(form: axum::Form<UserInput>) -> Html<String> {
    let start = Instant::now();

    let amount = form.amount;
    let accno = form.id;
    let ctgry = &form.category;
    let current_time = chrono::offset::Local::now().to_string();

    let mut user = read(accno);

    if user.balance < amount {
        let u = render!(FAILURE, user => user);

        let duration = start.elapsed();
        println!("WITHDRAW activity(failure)-time elapsed: {:?}", duration);

        return Html(u);
    } else {
        user.balance -= amount;
        
        user.history.insert(
            0,
            Transaction {
                amount,
                date: current_time,
                category: ctgry.to_string(),
                crdr: "dr".to_string()
            },
        );

        match ctgry.as_str() {
            "bills" => user.spending.bills += amount,
            "vacation" => user.spending.vacation += amount,
            "misc" => user.spending.misc += amount,
            "food" => user.spending.food += amount,
            &_ => println!("error"),
        }
    }
    
    let x = render!(SUCCESS, user => user);
    write(user, accno);

    let duration = start.elapsed();
    println!("WITHDRAW activity(success)-time elapsed: {:?}", duration);

    Html(x)
}

pub async fn assign(params: axum::extract::Path<String>) -> Html<String>{
    let start = Instant::now();

    let active_user = params.to_string();
    let accno:u32 = active_user.parse().unwrap();

    let user = read(accno);
    let s = render!(ASSIGN, user => user);

    let duration = start.elapsed();
    println!("rendering ASSIGN-time elapsed: {:?}", duration);

    Html(s)
}

pub async fn assignactivity(form: axum::Form<UserInput>) -> Html<String>{
    let start = Instant::now();

    let amount = form.amount;
    let accno = form.id;
    let ctgry = &form.category;

    let mut user = read(accno);

    if user.assigned_balance < amount {
        let u = render!(FAILURE, user => user);
        let duration = start.elapsed();
        println!("ASSIGN activtity(failure)-time elapsed: {:?}", duration);
        return Html(u);
    } else {
        match ctgry.as_str() {
            "bills" => user.assigned.bills_assigned += amount,
            "vacation" => user.assigned.vacation_assigned += amount,
            "misc" => user.assigned.misc_assigned += amount,
            "food" => user.assigned.food_assigned += amount,
            &_ => println!("error"),
        }
        user.assigned_balance -= amount;
    }

    let x = render!(ASSIGNSUCCESS, user => user);
    write(user, accno);

    let duration = start.elapsed();
    println!("ASSIGN activtity(success)-time elapsed: {:?}", duration);

    Html(x)
}

pub async fn delete(params: axum::extract::Path<String>) -> Html<String> {
    let start = Instant::now();

    let active_user = params.to_string();
    let accno:u32 = active_user.parse().unwrap();

    let user_path = format!("user-{}.json", accno);
    let file_path = user_path;

    match fs::remove_file(file_path) {
        Ok(()) => println!("File deleted successfully."),
        Err(err) => eprintln!("Error deleting file: {}", err),
    }

    let h = render!(REMOVESUCCESS);

    let duration = start.elapsed();
    println!("deleted a user's file-time elapsed: {:?}", duration);

    Html(h)
}