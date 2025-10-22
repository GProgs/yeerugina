use yeerugina::cmd::{Value,ValueKind};

fn main() {
    // Example code that demonstrates working with Values.
    // 1) Show the limits of color temperature, and create two Values.
    // 2)

    println!("Limit for color temp is {:#?}",Value::limit(ValueKind::ColorTemp));
}
