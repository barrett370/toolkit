use uuid::Uuid;

fn main() {
    println!(
        "{}",
        Uuid::new_v4().to_string()
    )
}
