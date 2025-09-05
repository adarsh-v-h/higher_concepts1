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
    ChnageColor(i32, i32, i32), 
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
