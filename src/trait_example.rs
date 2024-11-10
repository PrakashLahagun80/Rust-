pub trait Summary {
    fn summarize(&self)->String;
}

pub struct Article {
    pub headline: String,
    pub content: String
}

impl Summary for Article{
    fn summarize(&self) -> String{
        format!("{}: {}",self.headline,self.content)
    }
}

pub fn notify(item: &impl Summary){
    println!("Breaking news! {}",item.summarize());
}