use crate::parser::Token;
use std::collections::BTreeMap;
use crate::latexparser::TexToken;
pub fn tex_count(token: &TexToken, map: &mut BTreeMap<String,usize>){
    match token{
        TexToken::Command {name: s,args: v}=>{
                if let Some(value) = map.get_mut(&("Command:  ".to_string()+s)) {
                    *value += 1;
                } else {
                    map.insert("Command:  ".to_string()+s,1);
                }
                tex_vecop_count(v,map);
            }
            TexToken::Environment{content: v, .. } =>{
                tex_vec_count(v,map);
            }
            TexToken::BraceGroup(v) =>{
                tex_vec_count(v,map);
            }
            TexToken::BracketGroup(v) =>{
                tex_vec_count(v,map);
            }
            TexToken::SuperScript(v) =>{
                tex_count(v,map);
            }
            TexToken::SubScript(v) =>{
                tex_count(v,map);
            }
            TexToken::Symbol(s) =>{
                if let Some(value) = map.get_mut(&("Symbol :  ".to_string()+s)) {
                    *value += 1;
                } else {
                    map.insert("Symbol :  ".to_string()+s,1);
                }
            }
            TexToken::Word(s) =>{
                if let Some(value) = map.get_mut(&("Word   :  ".to_string()+s)) {
                    *value += 1;
                } else {
                    map.insert("Word   :  ".to_string()+s,1);
                }
            }
            _ =>{}
    }
}
pub fn tex_vecop_count(tokens: &Vec<Option<TexToken> >, map: &mut BTreeMap<String,usize>){
    let mut it = tokens.iter();
    while let Some(c) = it.next(){
        match c {
            None =>{},
            Some(w) =>{
                tex_count(w,map);
            }
        }
    }
}
pub fn tex_vec_count(tokens: &Vec<TexToken>, map: &mut BTreeMap<String,usize>){
    let mut it = tokens.iter();
    while let Some(c) = it.next(){
        tex_count(c,map);
    }
}
pub fn count(tokens: &Vec<Token>, map: &mut BTreeMap<String,usize>){
    let mut it = tokens.iter();
    while let Some(c) = it.next(){
        match c {
            Token::InlineMath(w) => {
                tex_vec_count(w,map);
            }
            Token::DisplayMath(w) => {
                tex_vec_count(w,map);
            }
            _ =>{}
        }
    }
}

