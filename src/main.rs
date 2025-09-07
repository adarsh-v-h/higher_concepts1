#[allow(dead_code)] // can use this to remove the warings, but its not recommended unless you are teaching someone something
fn main() {
    println!("Hello, world!");
    // Enums, they allow you to define a type by enumerating its possible variants.
    // enums give you a way of saying a value is one of a possible set.
    // now we can create insatances of these, like this
    let four = IpAdderkind::V4;
    let six = IpAdderkind::V6; 
    // we can call the function with either variants
    route(IpAdderkind::V4);
    route(IpAdderkind::V6);
    // we can create instances of this struct, and in place of IpAddrkind, we can have either one of its varints
    // let home = IpAddr{
    //     kind: IpAdderkind::V4,
    //     address: String::from("127.0.0.1")
    // };
    // let loopback = IpAddr{
    //     kind: IpAdderkind::V6,
    //     address: String::from("::1")
    // };
   // with associcated values in enum, we can create instances like this
   let home = IpAddr::V4(String::from("127.0.0.1"));
   let loopback = IpAddr::V6(String::from("::1"));
   let m = Message::Write(String::from("helloo"));
   m.call();
   // we called the method Write, the method can use Self to access those
    // next we will talk about option, its a enum defined by the standdar library, 
    // The Option type encodes the very common scenario in which a value could be something or it could be nothing.
    // this concept in terms of the type system means the compiler can check whether you have handled all the cases you should.
    // Using option
    let some_num = Some(5); // it is option <i32>
    let some_char = Some('e'); // it is option<char>
    let absent_num: Option<i32> = None; // assiging None to the absent_num
    // if we create a variable of i32 and try to add it with the some_num, where we gave type of i32, we will get error
    // because the type of some_num is not i32 but Option<i32> which is not same as just i32
    // and in case of some_num, we have a possibility of "None" to occur, and complier doesnt like that possibiltiy, so you are stopped at the complie time itself
    // so you have to convert Option<T> to just T 
    // Now want a set of instructions to run if Some(T) is the variant and some different instruction to run for None as variant, for this we use match expression.
    // We have a set coins, and we will use match to print out a certain set of code for a particaular coin
    enum Coin { // to hold all the types of coin
        Penny,
        Nickel,
        Dime,
        Quarter(UsState)
    }
    
    fn value_in_cents(coin: Coin) ->u8{ // will take the variant of Coin
        match coin{ // is checking of the variant in it, while the type of all the vairants is same, i.e Coin enum
            Coin::Penny => 1, // this is an arm, sperated by an operator =>, the first part is pattern and the next is code to run if matched
            Coin::Nickel =>{ // need to use {} for multiple statements
                println!("Nickel coin not element, hahah"); // if matched, this is executed
                5 // then returns this
            },
            Coin::Dime => 10,
            Coin:: Quarter(state) =>{ //Matches the Quarter variant and extracts the attached UsState into the variable state.
                println!("State quarter from {:?}", state); // :? to 
                25
                }
        } // will return the value of that coin in cents
    }

    let sec_coin = Coin::Nickel; // a variant is stored in sec_Coin
    let true_val = value_in_cents(sec_coin); // the retured value from fn is stored in this
    println!("The value of the secret coin is: {} cents", true_val);
    // note: match might look similar to if, but if needed a bool value but match can work with any type
    // we can create an enum which can be valued in another enum
    #[derive(Debug)] //  without this you couldnt use :? or :#? on this enum
    enum UsState{
        Alabama,
        Alaska
    }
    let sec_coin1 = Coin::Quarter(UsState::Alabama);
    let true_val1 = value_in_cents(sec_coin1);
    println!("The quatar: {:#?} cents", true_val1);
    // Now lets try to use match to handle option<T>, dont forget, T here is just data type
    /*We will create a function which takes input of type Option<i32> and returns the same,
    since we are using it from preloaded module, we wont have to define it again.*/
    fn plus_one(x: Option<i32>) -> Option<i32>{
        match x{
            None => None,
            Some(i) => Some(i+1) 
            // Some will take a value and add one to it
        }
    }
    // lets start with creating a variable of type Option<i32>
    let five = Some(5);
    let six = plus_one(five); // in place of x we are sending five, which hold Some(5), 
    // which will then go into the match and go to the some(i) arm, and then it becomes Some(6),
    // which is then returned to var six
    let none = plus_one(None);
    // x will be None type and it will return None
    println!("{:?}", none); // prints None
    println!("{:?}", six); // prints some(6)
    // not that in match the arm's patterns should cover all possibilities,
    // this is called being exhaustive, which means we have to exhaust all possibilities 
    //We can also have a default value similar, if all the above conditions didnt match, just run this for any other possible value
    let dice_ = 9;
    match dice_ {
        3 => println!("Dont move"),
        7 => println!("Role once more"),
        other => println!("move {} steps", dice_), // takes care of any other condition thats left
    }

}
// Lets take an example, lets say, we listing out ips, they can either be v4 or v6, either one not both,
    // we can create a enu for this
enum IpAdderkind {
    V4,
    V6,        
}
// Now we also can define a function that takes IpAddrKind
fn route(ip_kind: IpAdderkind) {}
// we can create a Struct that stores the IP,
// struct IpAddr{
//         kind: IpAdderkind,
//         address: String
//}
 // now this same thing we can do it without struct, and mentioning in the enum definition itself that both variants are associated with.
enum IpAddr{
    V4(String),
    V6(String)
}
// this requirement is so common, we hane built in standard library for this called, IpAddr
// since we haven't yet imported it, there is no conflict between built in and user defined enums
// Lets look at enum which has a varity of associated values
enum Message{
    Quit,
    Move {x: i32, y: i32},
    Write(String),
    ChangeColor(i32, i32, i32), 
}
// to create the same with struct, we would have to do this
struct QuitMessage; // unit struct
struct MoveMessage{
    x: i32,
    y: i32
}
struct WriteMessage(String); // tuple struct
struct ChnageColorMessage(i32, i32, i32); // tuple struct
// but with these many vairty of structs, it would be hard to create a function
// in enum we can also use impl to create methods, just like struct,
impl Message {
    fn call(&self){
        // method body
    }
} // The body of the method would use self to get the value that we called the method on
// self will be Message::Write(String::from("hello")), this will be the body of call
//-------------------------------------------------------------------------------
// Rust doesnt has the value null, below is a way how rust uses null with Option<T>
// enum Option<T>{ // its preloaded no need to bring it in explictly
//     /*<T> means that the Some variant of the Option enum can hold one piece of data of any type,
//     and that each concrete type that gets used in place of T makes the overall Option<T> type a different type. */
//     Some(T),
//     None
// } even if i comment this out, i will still be able to use Option<T>, because of its standard lib being aviable