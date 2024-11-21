const EMPTY_STRING: String = String::new();


fn main() {
    test_map();

}

fn test_map() {
    let mut hash_map = HashMap::new();
    hash_map.put(784, "Test".to_string());
    let result = hash_map.get(784);
    println!("{result}");
    if result == "".to_string() {
        println!("Missing element, make sure you have the right key")
    }
}

struct HashMap {
    size: i32,
    base_array: [String; 100],
}

impl HashMap {
    fn new() -> Self {
        Self { size: 99, base_array: [EMPTY_STRING;100] }
    }

    fn put(&mut self, key: i32, value:String) {
        self.base_array[(key % self.size)as usize] = value;
    }

    fn get(&mut self, key: i32)->String {
        let ret_val = self.base_array.get((key % self.size) as usize);
        ret_val.unwrap().to_string()
    }
}