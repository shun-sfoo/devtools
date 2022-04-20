use uuid::Uuid;

fn main() {
    let id = Uuid::new_v4();
    println!("{}", id);
    let arr: Vec<String> = Vec::new();
    let _brr: Vec<String> = arr
        .into_iter()
        .filter(|a| a == &"hello".to_string())
        .collect();
}
