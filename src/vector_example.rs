pub fn even_filter(vec:Vec<i32>)-> Vec<i32>{
  let mut numbers = Vec::new();
   for value in vec {
    if value %2 ==0{
        numbers.push(value);
    }
   }
   return numbers;
}