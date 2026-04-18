use redis::Value;

fn main() {
    let v = Value::Nil;
    match v {
        Value::Nil => println!("Nil"),
        _ => println!("Other"),
    }
}
