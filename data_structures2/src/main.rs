fn main() {
    test_map()
}

fn test_map() {
    let mut hash_map = HashMap::new(150);
    hash_map.put(784, "Test".to_string());
    let result = hash_map.get(784);
    println!("{result}");
    if result == "".to_string() {
        println!("Missing element, make sure you have the right key")
    }
}
struct HashMap{
    data: Vec<String>,
    size:usize,

}

impl HashMap{

    fn new(size:usize) -> Self {
        Self { data : vec![String::new();size ], size }
    }

    fn put(&mut self, key:usize, value:String){
        let index = self.hash_function(key);
        self.data[index] = value;
    }

    fn get(&mut self, key:usize)->String{
        let index = self.hash_function(key);
        self.data[index].clone()
    }

    fn hash_function(&self,key:usize)->usize{
        key % self.size
    }
}