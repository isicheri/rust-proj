use std::collections::HashMap;

pub struct HashMapStruct { name: String,score: i32}

impl HashMapStruct {
     pub fn default(name:String,score:i32) -> Self{
        HashMapStruct {name,score}
     }
     pub fn new(&self) -> HashMap<String,i32> {
        let mut user_map:HashMap<String,i32> = HashMap::new();
        user_map.insert(self.name.clone(),  self.score);
        return user_map;
     }
}