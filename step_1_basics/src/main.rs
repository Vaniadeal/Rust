
// fn main(){
//     let user_name = "Иван";
//     let mut balance = 1000;

//     println!("Привет {user_name}, стартовый баланс: {balance}");

//     balance -= 250;
//     println!("Произошла покупка.Итоговый баланс: {balance}");
// }

/*
fn make_purchase(balance: i32, price: i32) -> i32{
    if balance >= price {
        println!("Покупка успешна на сумму {price}");
        balance - price
    }else{
        println!("Недостаточно денег на покупку {price}!");
        balance
    }
}

fn main(){
    let mut balance = 1000;

    balance = make_purchase(balance, 400);
    balance = make_purchase(balance, 800);

    println!("Финальный баланс: {balance}");
}
*/

/*
fn print_user(name: &String){
    println!("Пользователь: {name}");
}

fn main(){
    let user = String::from("Иван");
    print_user(&user);
    print_user(&user);
} 

 */



/*
fn print_user(name: &str){
    println!("Обычный пользователь: {name}")
}

fn make_vip(name: &mut String){
    name.push_str("[VIP]")
}

fn main(){
    let mut user = String::from("Иван");
    print_user(&user);
    make_vip(&mut user);
    println!("Итоговый статус пользователя: {user}");
}  */

/*
struct User{
    name: String,
    balance: i32,
    discount: Option<i32>
}


impl User{

    fn new(name: String, balance: i32, discount: Option<i32>) -> Self{
        Self {name, balance, discount}
    }

    fn print_info(&self){
        println!("Пользователь: {}, баланс :{}", self.name, self.balance);
    }

    fn print_discount(&self){
        if let Some(percent) = &self.discount{
            println!("Скидка пользоватля{percent}%")
        }
    }

    fn buy(&mut self, prise: i32) -> Result<i32, String>{
        if self.balance >= prise {
            self.balance -= prise;
            Ok(self.balance)
        }else{
            let diff = prise - self.balance;
            Err(format!("Недостаточно средств {diff} руб."))
        }
    }
}

fn checkout(user: &mut User) -> Result<i32, String> {
    user.buy(300)?;
    user.buy(500)?;
    Ok(user.balance)
}

fn main(){
    let mut user= User::new(String::from("Иван"), 1000, Some(15));

    user.print_info();

    match checkout(&mut user){
        Ok(new_balance) => {
            println!("Все покупки успешны! Новый баланс {new_balance}")
        }
        Err(e) => {
            println!("Заказ отменен : {e}")
        }
    }


    user.print_discount();


} */


/*

fn main(){
    const MAX_SCORE: u32 = 100;
    let mut score = 10;

    println!("Старт: {score}");

    score += 15;

    println!("После старта: {score}");

    {let score = score *2; println!("Внутри блока: {score}");}

    println!("После блока: {score}");

    let user = ('A', 25, true);

    let nums = [1, 4, 5, 6, 7, 4];

    println!("Первый элемент кортежа: {}, Первый элемент массива: {}", user.0, nums[0]);


}

 */


/*

fn check_score(score: i32) -> i32{
    if score >= 10 {
        score * 2
    }else{
        score + 5
    }
}


fn main(){
    let scores = [5, 10, 15, 20 ];

    for s in scores  {
        println!("{}", check_score(s));
    }

    let mut count = 0;
    let bonus = loop { count += 1 ; if count == 3 {break count * 10}};

    println!("Бонус: {bonus}");
}

*/


/*
fn convert_temp(value: f64, unit: char) -> f64 {
    if unit == 'F' {
        (value - 32.0) / 1.8
    }else if unit == 'C' {
        value * 1.8 + 32.0
    }else{
        println!("Неизвестные единицы измерения ");
        value
    }
}

fn main(){
    let temps = [0.0, 20.0, 36.6, 100.0];

    for value in temps{
        println!("Фаренгейты: {}, Цельсии: {}", value, convert_temp(value, 'F'));
    }
}
*/

/*
fn main(){
    let a = 10;
    let b = a;
    println!("{a} И {b}");

    let s1 = String::from("Rust");
    let s2 = s1.clone();
    println!("{s1} and {s2}");

    show_text(s2.clone());
    println!("{}", s2);
}

fn show_text(text: String) {
    println!("Функция получила: {text}");
}
*/


/*


fn main(){
    let mut message = String::from("Rust");

    show_text(&message);
    println!("{message}");

    add_exlamation(&mut message);
    show_text(&message);

    let r1 = &message;
    let r2 = &message; 

    println!("{r1} and {r2}");

    let r3 = &mut message;
    r3.push_str("!");
    println!("{r3}");

    greet("Ivan");

    let person = String::from("Vadim");
    greet(&person);

    let text = String::from("Hello world");
    let word = &text[..5];
    println!("{word}");
    
}

fn show_text(text: &String){
    println!("{text}");
}

fn add_exlamation(text: &mut String){
    text.push_str("!");
}


fn greet(name: &str){
    println!("Привет {name}");
}
*/




/*
#[derive(Debug)]
struct Player{
    name: String,
    hp: i32,
    score: u32,
}


impl Player{

    fn new(name: &str ) -> Player{
        Player {name: String::from(name), hp: 100, score: 0}
    }

    fn is_alive(&self) -> bool {
        self.hp > 0
    }

    fn take_damage(&mut self, damage: i32) {
        self.hp -= damage;
    }
}

fn main(){
    let mut p = Player::new("Hunter");

    p.take_damage(30);

    println!("{:#?}", p);

    println!("Жив {} ? - {}", p.name, p.is_alive());

}
*/





/*

#[derive(Debug)]
enum Status{
    Active,
    Paused,
    Offline,
}

fn print_status(status: Status){
    match status {
        Status::Active => {
            println!("Пользователь в сети");
        }
        Status::Paused => {
            println!("Отошел");
        }
        Status::Offline => {
            println!("Не в сети");
        }

    }
}

fn check_bonus(score: u32) -> Option<u32>{
    if score >= 100 {
        Some(score * 2)
    }else{
        None
    }
}

fn main(){
    if let Some(val) = check_bonus(150){
        println!("Бонус = {val}")
    }else{
        println!("Бонуса нет")
    }
}
*/


/*
mod restaurant;

use crate::restaurant::serve_customer;

fn test_restaurant(){

    serve_customer();
}


fn main(){
    test_restaurant();

}

*/


/*
fn test_vectors(){
    let mut numbers = vec![10, 20, 30];

    numbers.push(40);
    numbers.push(50);

    for n in &mut numbers{
        *n += 5;
    }

    if let Some(val) = numbers.get(1){
        println!("Элемент с и6ндексом 1: {val}");
    }

    println!("{numbers:?}")

}

fn main(){
    test_vectors();
}

*/


/*
use std::collections::HashMap;

fn test_hashmap(){
    let text = "hello world wonderful werld";
    let mut inventory = HashMap::new();

    inventory.insert(String::from("яблоки"), 10);
    inventory.insert(String::from("бананы"), 20);

    inventory.entry(String::from("яблоки")).or_insert (20);

    if let Some(count) = inventory.get("бананы"){
        println!("Колличество бананов инвентаре {count}");
    }

    for (item, count) in &inventory {
        println!("{item} : {count}");
    }
}

fn main(){
    test_hashmap();
}
*/




/*
fn divide(a: f64,  b: f64) -> Result<f64, String> {
    if b == 0.0 {Err(String::from("Деление на ноль   запрещено!"))}
    else {Ok(a/b)}
}

fn main(){
    match divide(10.0, 2.0) {
        Ok(val) => println!("Результат {val}"),
        Err(err) => println!("Ошибка: {err}")
    }

    if let Err(err) = divide(10.0, 0.0) {println!("Ошибка: {err}")}
}
*/

#[derive(Debug)]
struct Container<T> {
    value: T,
}

impl<T> Container<T>{
    fn new(value: T) -> Container<T>{
        Container {value}
    }

    fn get_value(&self) -> &T{
        &self.value
    }
}

fn test_generics(){
    let int_c = Container::new(42);

    let str_c = Container::new(String::from("Rust"));

    println!("Число {}", int_c.get_value());
    println!("Строка {}", str_c.get_value());
}

fn main(){
    test_generics();
}