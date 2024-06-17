pub trait Summary {
    fn summarize(&self) -> String;
}
pub struct Weibo {
    pub username: String,
    pub content: String,
}
impl Summary for Weibo {
    fn summarize(&self) -> String {
        format!("{}发表了微博{}", self.username, self.content)
    }
}
// 特征约束作为参数
pub fn notify(item: &impl Summary) {
    println!("Breaking news! {}", item.summarize());
}
