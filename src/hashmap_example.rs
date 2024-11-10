use std::collections::HashMap;

pub fn group_by_user(vec :Vec<(String,i32)>)-> HashMap<String, i32>{
    let mut new_hash = HashMap::new();
    for (key,value) in vec {
        new_hash.insert(key,value);
    }
    return new_hash;
}
