use crate::latexparser::TexToken;
pub fn tex_format(token: TexToken, tag: &str)-> TexToken{
    match token{
        TexToken::Command {name: s,args: v}=>{
            if tag == "command"{//一时半会真是没想到什么command需要改，只能先放在这里了
                match s.as_str() {
                    _ => TexToken::Command{name: s, args: tex_vecop_format(v,tag)},
                }
            }else {
                if tag == "word" && (s.starts_with("text")||s.starts_with("math")){
                    TexToken::Command{name: s, args: tex_vecop_format(v,"normal")}
                }else {
                    TexToken::Command{name: s, args: tex_vecop_format(v,tag)}
                }
            }
        }
        TexToken::Environment{name: s,content: v } =>{
            TexToken::Environment{name: s, content: tex_vec_format(v,tag)}
        }
        TexToken::BraceGroup(v) =>{
            if tag == "command" && (!v.is_empty()){
                if v[0] == TexToken::Symbol("rm".to_string()) {
                    let mut v2 = v;
                    v2.remove(0);
                    return TexToken::Command{name: "mathrm".to_string(),args: vec![Some(TexToken::BraceGroup(v2))]};
                }
                else {
                    TexToken::BraceGroup(tex_vec_format(v,tag))
                }
            }else {
                TexToken::BraceGroup(tex_vec_format(v,tag))
            }
        }
        TexToken::BracketGroup(v) =>{
            TexToken::BracketGroup(tex_vec_format(v,tag))
        }
        TexToken::SuperScript(v) =>{
            TexToken::SuperScript(Box::new(tex_format(*v,tag)))
        }
        TexToken::SubScript(v) =>{
            TexToken::SubScript(Box::new(tex_format(*v,tag)))
        }
        TexToken::Symbol(s) if tag == "symbol"=>{
            match s.as_str(){
                "and" => TexToken::Symbol("land".to_string()),
                "lang" => TexToken::Symbol("langle".to_string()),
                "rang" => TexToken::Symbol("rangle".to_string()),
                "or"  => TexToken::Symbol("lor".to_string()),
                "empty" => TexToken::Symbol("emptyset".to_string()),
                "N"|"Z"|"Q"|"R"|"C"     => TexToken::Command { name: "mathbb".to_string(), args: vec![Some(TexToken::BraceGroup(vec![TexToken::Word(s)]))] },
                "sdot"  => TexToken::Symbol("cdot".to_string()),
                "exist"  => TexToken::Symbol("exists".to_string()),
                _ => TexToken::Symbol(s),
            }
        }
        TexToken::Word(s) =>{
            if tag == "word"{
                match s.as_str(){
                    "Hom"|"Span"|"Stab"|"univ"|"unit"|"sym"|"odd"|"even"|"coker"|"char"|"Char"|"Trace"|"Min"|"Mul"|"Bil"|"Frac"|"prime"
                        => TexToken::Command { name: "mathrm".to_string(), args: vec![Some(TexToken::BraceGroup(vec![TexToken::Word(s)]))] },
                    "otherwise"|"Otherwise" 
                        => TexToken::Command { name: "text".to_string(), args: vec![Some(TexToken::BraceGroup(vec![TexToken::Word(s)]))] },
                    _ => TexToken::Word(s),
                }
            } else {
                TexToken::Word(s)
            }
        }
        _ =>{
            token
        }
    }
}
pub fn tex_vecop_format(tokens: Vec<Option<TexToken> >, tag: &str)-> Vec<Option<TexToken> >{
    // let mut it = tokens.iter();
    let mut ans = Vec::new();
    for c in tokens {
        match c {
            None =>{ans.push(None);},
            Some(w) =>{
                ans.push(Some(tex_format(w,tag)));
            }
        }
    }
    return ans;
}
pub fn tex_vec_format(tokens: Vec<TexToken>, tag: &str) -> Vec<TexToken>{
    let mut ans = Vec::new();
    for c in tokens {
        ans.push(tex_format(c,tag));
    }
    return ans;
}
use crate::parser::Token;
pub fn format_in(tokens: Vec<Token>, tag: &str) -> Vec<Token>{
    let mut ans = Vec::new();
    for c in tokens{
        match c{
            Token::InlineMath(w) =>{
                ans.push(Token::InlineMath(tex_vec_format(w,tag)));
            }
            Token::DisplayMath(w) =>{
                ans.push(Token::DisplayMath(tex_vec_format(w,tag)));
            }
            _ =>{
                ans.push(c);
            }
        }
    }
    return ans;
}
pub fn format(tokens: Vec<Token>) -> Vec<Token>{
    return format_environment(
        format_in(
            format_in(
                format_in(tokens,"command"),
                "symbol"
            ),
            "word"
        ),
    );
}
fn check_cmp(token: &TexToken)->bool{
    match token{
        TexToken::Other('=')|TexToken::Other('<')|TexToken::Other('>') =>true,
        TexToken::Symbol(s) if s == "leq"|| s=="equiv"||s=="geq"||s=="sim"||s=="approx"
        ||s=="to"||s=="rightarrow"||s=="Leftrightarrow"||s=="Rightarrow"
        ||s=="in"||s=="ne"||s=="notin"||s=="cong"||s=="mapsto"
            =>true,
        _ =>false,
    }
}
fn check_is_eq(tokens: &Vec<TexToken>) -> bool{
    for c in tokens{
        if check_cmp(c){
            return true;
        }
    }
    return false;
}
fn check_is_enter(tokens: &Vec<TexToken>) -> bool{
    for c in tokens{
        match c{
            TexToken::Symbol(s) if s == "\\" =>{return true;}
            _ =>{}
        }
    }
    return false;
}
fn check_enter_first(tokens: &Vec<TexToken>) -> bool{
    for c in tokens{
        match c{
            TexToken::Symbol(s) if s == "\\" =>{return true;}
            _ if check_cmp(c)=>{return false;}
            _ => {}
        }
    }
    return false;
}
fn check_no_environment(tokens: &Vec<TexToken>) -> bool{
    return check_is_enter(tokens);
}

fn make_aligned_two(tokens: Vec<TexToken>) -> Vec<TexToken>{
    let mut ans=Vec::new();
    let mut flag = true;
    for c in tokens{
        if check_cmp(&c) && flag{
            ans.push(TexToken::Other('&'));
            flag = false;
        }
        if c == TexToken::Symbol("\\".to_string()){
            flag=true;
        }
        ans.push(c);
    }
    return ans;
}
fn make_aligned_one(tokens: Vec<TexToken>) -> Vec<TexToken>{
    let mut ans=Vec::new();
    ans.push(TexToken::Other('&'));
    let mut flag = true;
    for c in tokens{
        if check_cmp(&c) && flag{
            ans.push(c);
            ans.push(TexToken::Other('&'));
            flag = false;
        } else{
            if c == TexToken::Symbol("\\".to_string()){
                flag=true;
            }
            ans.push(c);
        }
    }
    return ans;
}
pub fn format_environment(tokens: Vec<Token>) -> Vec<Token>{
    let mut ans = Vec::new();
    for c in tokens{
        match c{
            Token::DisplayMath(w) =>{
                if check_no_environment(&w){
                    if check_is_eq(&w){
                        if check_enter_first(&w){
                            ans.push(Token::DisplayMath(vec![TexToken::Environment { name: "aligned".to_string(), content: make_aligned_one(w) }]));
                        }else {
                            ans.push(Token::DisplayMath(vec![TexToken::Environment { name: "aligned".to_string(), content: make_aligned_two(w) }]));
                        }
                    }else {
                        ans.push(Token::DisplayMath(vec![TexToken::Environment { name: "gathered".to_string(), content: w }]));
                    }
                }else {
                    ans.push(Token::DisplayMath(w));
                }
            }
            _ =>{
                ans.push(c);
            }
        }
    }
    return ans;
}

