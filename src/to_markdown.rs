use crate::parser::Token;
pub fn tomarkdown(tokens: Vec<Token>) -> String {
    let mut ans = String::new();
    let mut it = tokens.iter();
    while let Some(t) = it.next() {
        ans.push_str(&t.to_string());
    }
    return ans;
}
use std::collections::BTreeMap;
use crate::counter;
use crate::to_markdown;
use crate::formatter;
use crate::parser;
pub fn parse_markdown(input: &str) -> String {
    let tokens = formatter::format(parser::parse(input.trim()));
    let mut map: BTreeMap<String,usize> = BTreeMap::new();
    counter::count(&tokens,&mut map);
    let new_article = to_markdown::tomarkdown(tokens);
    for (key, value) in &map {
        println!("{}: {}", key, value);
    }
    return new_article;
}