pub mod article;
pub mod cheatsheet;


pub trait Parser{
    fn parse(markdown: &str) -> Self where Self: Sized;
}