/*fn main(){

}

fn swap<T: Copy>(x:&mut T, y:&mut T){
    let temp =*x;
    *x=*y;
    *y=temp;

}


trait Summary{
    fn summarize (&self) ->String;
}

fn print_summary<T:Summary>(item:T){

    println!("{}",item.summarize());
}

fn print_double_summary<T,U>(item1:T,item2:U)
where
T:Summary
U:Summary+Clone,
{
    println!("{}",item1.summarize());
    println!("{}",item2.summarize());
    let cloned_item=item2.Clone();
    println!("{}" , cloned_item.summarize());
}

*/


/*
use std ::collections::HashMap;


fn main(){
 let my_map=HashMap:.new();

 my_map.insert(k:"Alice".to_string(),v:10);
 my_mapçinsert(k:"Bob".to_string(),v:20);
 for (key,value) in my_map.iter(){
    println!("{} has {}",key,value);
 }
}*/
/*
use std::collections::HashMap;
fn main(){
    let numbers=vec![1,2,3,4,5];
   /* let doubled: Vec<i32>=numbers.iter().map(|x| x*2).collect();
println!("{:?}",doubled);*/


/*
let even_numbers: Vec<i32>=numbers.into_iter().filter(|x| x % 2==0).collect();
println!("{:?}",even_numbers);
}
*/

/*
let sum:i32=numbers.iter().fold(init:0, f:|acc,x|acc+x);
println!("{}",sum);

*/


/*



 let chained: Vec<i32> = numbers
 .into_iter()               // Vec'i bir iteratora çeviriyoruz
 .filter(|n| n % 2 == 0)     // Yalnızca çift sayıları filtreliyoruz
 .map(|n| n * 2)             // Her bir çift sayıyı 2 ile çarpıyoruz
 .collect();                 // Sonuçları bir Vec olarak topluyoruz

println!("{:?}", chained);      // Sonucu ekrana yazdırıyoruz
}


*/

let squared_numbers: HashMap<_, _>= numbers.iter().map(|n|(n,n*2)).collect();
println!("{:?}",squared_numbers);

}


/*
fn multiply(x:i32)->i32{
    x*2
}*/


*/
/*use sports::football;
fn main(){
 

   football();

   
    /*let messi=sports::footballPlayer{
        name:"Messi".to_string(),
        age:19,
    };
*/


}*/

/*
mod sports; // sports modülünü dahil et

use sports::{football, FootballPlayer};  // Fonksiyon ve struct'ı dahil et

fn main() {
    let my_player = FootballPlayer {
        name: "Simon".to_string(),
        age: 26,
    };
    
    football();
}


*/
/*
//   ****traits and Generics******

fn main(){
    let dog=Dog{
        name:"Rudolf".to_string(),
    };
    //dog.speak();

    let cow=Cow{
        name:"Not Rudolf".to_string(),
    };
    //cow.speak();

    let original_string=String::from("This is original");
    let copy_string=original_string.display();

   // println!("{}",copy_string);

    animal_speaks(&cow);
    animal_speaks(&dog);
   
    let cat=Cat;
    cat.make_sound();
    cat.walk();
    cat.sleep();





}







trait Speak{
    fn speak(&self);
}
Option<f64>{
struct Dog{
    name:String,
}
struct Cow{
    name:String,
}

impl Speak for Dog{
    fn speak(&self){
        println!("{} says Woof!",self.name);
    }
}
impl Speak for Cow{
    fn speak(&self){
        println!("{} says Mooo!",self.name);
    }
}

trait Display{
    fn display(&self)->String;
}

impl Display for String{
    fn display(&self) ->String {
        self.clone()
    }
}

fn animal_speaks<T: Speak>(animal:&T){
    animal.speak();
}

trait Animal{
    fn make_sound(&self);
    fn sleep(&self){
        println!("Animals sleep......");
    }
}
trait Mammal:Animal{
    fn walk(&self);
}

trait Bird:Animal{
    fn fly(&self);
}

struct Cat;
impl Animal for Cat{
    fn make_sound(&self){
        println!("Miyavvvvv");
    }
}

impl Mammal for Cat{
    fn walk(&self){
        println!("The cat is walking");
    }
}

*/


// *****erorrr handlingggg*****

use std::{fs, println};

fn main() {
    let my_content = getFileContent("my_file.txt");

    match my_content {
        Ok(item) => println!("the result is {}", item),
        Err(_) => println!("We got an error"),
    }
}

fn getFileContent(file_name: &str) -> Result<String, std::io::Error> {
    let content = fs::read_to_string(file_name)?;

    Ok(content)
}

fn divide(numerator: f64, denominator: f64) -> Option<f64> {
    if denominator == 0.0 {
        return None;
    } else {
        return Some(numerator / denominator);
    }
}
