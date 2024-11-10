pub fn filter_map_example(vec :Vec<i32>) -> Vec<i32>{
    let new_itr = vec.iter().filter(|x| *x%2==1).map(|x| x+1);
    let new_vect :Vec<i32> = new_itr.collect();
    return new_vect;
}