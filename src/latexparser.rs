#[derive(Debug,PartialEq)]
pub enum TexToken {
    Command {
        name: String,
        args: Vec<Option<TexToken>>,
    },
    Environment{
        name: String,
        content: Vec<TexToken>,
    },
    BraceGroup(Vec<TexToken>),
    BracketGroup(Vec<TexToken>),
    SuperScript(Box<TexToken>),
    SubScript(Box<TexToken>),
    Symbol(String), //转义符或者\lambda这种单个字符
    Comment(String),
    Other(char),
    AtModifier(String),//@跟的运算符

    Blank,
    Word(String),
    Number(String),
    //这两个的作用是最后再搞得，一开始只是把所有的都分开，最后会在连续的Other中挑选相邻的字符/数字拼起来
}
impl TexToken {
    #[allow(dead_code)]
    fn check_token(&self){
        match self{
            Self::BraceGroup(..) | Self::Symbol(..) | Self::Other(..) => {},
            _ =>{panic!("并不是一个token!");}
        }
    }
    fn check_brace(&self){
        match self{
            Self::BraceGroup(..)  => {},
            _ =>{panic!("并不是一个brace!");}
        }
    }
    fn tostring(&self,tag: bool) -> String {//false:inline true:display
        match self {
            Self::Symbol(s) => {
                if tag && s == "\\"{
                    return "\\".to_string() + s + "\n";
                }
                else {
                    return "\\".to_string() + s;
                }
            }
            Self::Environment { name, content } =>{
                if tag{
                    return "\\begin".to_string()+"{"+name+"}\n"+&TexToken::vectostring(content,tag)+"\n\\end"+"{"+name+"}";
                }else {
                    return "\\begin".to_string()+"{"+name+"}"+&TexToken::vectostring(content,tag)+"\\end"+"{"+name+"}";
                }
            }
            Self::Word(s) => s.to_string(),
            Self::Other(c) => (*c).to_string(),
            Self::Number(s) => s.to_string(),
            Self::Comment(s) => {
                if tag {
                    return "%".to_string() + s + "\n";
                }
                else {
                    return "%".to_string() + s;
                }
            }
            Self::Command { name: s, args: t } => {
                let mut ans = "\\".to_string() + s;
                let w = TexToken::vecoptostring(t,tag);
                if let Some(c) = w.chars().next(){
                    if c == '{' || c == '[' {}
                    else {
                        ans.push(' ');
                    }
                }
                return ans + &w;
            }
            Self::BraceGroup(t) => "{ ".to_string() + &(TexToken::vectostring(t,tag)) + " }",
            Self::BracketGroup(t) => "[".to_string() + &(TexToken::vectostring(t,tag)) + "]",
            Self::SuperScript(t) => "^".to_string() + &t.tostring(tag),
            Self::SubScript(t) => "_".to_string() + &t.tostring(tag),
            Self::Blank => " ".to_string(),
            Self::AtModifier(s) => "@{".to_string()+s+"}",
        }
    }
    pub fn vectostring(d: &Vec<TexToken>,tag:bool) -> String {
        let mut ans = String::new();
        let mut it = d.iter();
        while let Some(wd) = it.next() {
            if (!ans.is_empty())&&(!ans.ends_with(" "))&&(!ans.ends_with("\n")){
                match &wd{
                    Self::Word(..)|Self::Number(..)|Self::Other(..)|Self::Command{..}|Self::Symbol(..)|Self::Environment {..} => ans.push(' '),
                    _ =>{}
                }
            }
            ans.push_str(&wd.tostring(tag));
        }
        return ans;
    }
    pub fn vecoptostring(d: &Vec<Option<TexToken>>,tag: bool) -> String {
        let mut ans = String::new();
        let mut it = d.iter();
        while let Some(wd) = it.next() {
            match wd {
                Some(s) => {
                    if (!ans.is_empty())&&(!ans.ends_with(" "))&&(!ans.ends_with("\n")){
                        match &s{
                            Self::Word(..)|Self::Number(..)|Self::Other(..) => ans.push(' '),
                            _ =>{}
                        }
                    }
                    ans.push_str(&s.tostring(tag));
                }
                None => {}
            }
        }
        return ans;
    }
    pub fn get_word(it: &mut std::iter::Peekable<std::str::Chars>) -> Option<String> {
        let mut ans = String::new();
        while let Some(c) = it.peek() {
            match c {
                'a'..='z' | 'A'..='Z' => {
                    ans.push(*c);
                }
                _ => {
                    if ans.is_empty() {
                        return None;
                    } else {
                        return Some(ans);
                    }
                }
            }
            it.next();
        }
        if ans.is_empty() {
            return None;
        } else {
            return Some(ans);
        }
    }
    pub fn get_line(it: &mut std::iter::Peekable<std::str::Chars>) -> String {
        let mut ans = String::new();
        while let Some(c) = it.peek() {
            match c {
                '\n' => {
                    return ans;
                }
                _ => {
                    ans.push(*c);
                }
            }
            it.next();
        }
        return ans;
    }
    fn parser_one(it: &mut std::iter::Peekable<std::str::Chars>) -> Self {
        // println!("start parser_one");
        if let Some(c) = it.next() {
            match c {
                '{' => {
                    return Self::BraceGroup(Self::parser(it, "Brace"));
                }
                '}' => {
                    panic!("大括号匹配错误");
                }
                // '[' if tag == "Bracket" => {
                //     return Self::BracketGroup(Self::parser(it, "Bracket"));
                // }
                // ']' if tag == "Bracket" => {
                //     panic!("中括号匹配错误");
                // }
                '^' => {
                    return Self::SuperScript(Box::new(Self::must_parser_one(it)));
                }
                '_' => {
                    return Self::SubScript(Box::new(Self::must_parser_one(it)));
                }
                '\\' => {
                    let word = Self::get_word(it);
                    match word {
                        Some(s) if s == "frac" || s=="binom" || s == "cfrac" => {
                            // println!("start frac\n");
                            let a = Self::must_parser_one_brace(it);
                            // println!("第一个已经拿到!\n");
                            // println!("{:?}\n",a);
                            let b = Self::must_parser_one_brace(it);
                            return Self::Command {
                                name: s,
                                args: vec![Some(a), Some(b)],
                            };
                        }
                        Some(s) if s == "sqrt" => {
                            let a = Self::try_parser_one_bracket(it);
                            let b = Self::must_parser_one_brace(it);
                            return Self::Command {
                                name: "sqrt".to_string(),
                                args: vec![a, Some(b)],
                            };
                        }
                        Some(s) if s == "begin" => {
                            let c = Self::get_text(it);//返回的是一个String
                            let w = Self::parser(it, &("".to_string()+&c+"_end"));
                            return Self::Environment {name: c, content: w};
                        }
                        Some(s) if s == "end" => {
                            let c = Self::get_text(it);//返回的是一个String
                            return Self::Environment {name: c+"_end", content: Vec::new()};
                        }
                        Some(s) if s.starts_with("text") => {
                            let c = Self::must_parser_one_brace(it);
                            // Self::check_token(&c);
                            return Self::Command{name: s,args: vec![Some(c)]};
                        }
                        Some(s) if s.starts_with("math") => {
                            let c = Self::must_parser_one_brace(it);
                            return Self::Command{name: s,args: vec![Some(c)]};
                        }
                        Some(s) if s == "tilde" || s == "vec" || s == "bar" || s == "check" || s =="underline" || s == "overline" || s == "pmod"=> {
                            let c = Self::must_parser_one_brace(it);
                            return Self::Command{name: s,args: vec![Some(c)]};
                        }
                        Some(s) if s.starts_with("xymatrix") => {
                            let c = Self::must_parser_one(it);
                            Self::check_brace(&c);
                            return Self::Command{name: s,args: vec![Some(c)]};
                        }
                        Some(s) if s == "ar" => {
                            let a = Self::must_parser_one(it);
                            match &a {
                                Self::AtModifier(..) => {
                                    let b = Self::must_parser_one_bracket(it);
                                    return Self::Command {
                                        name: "ar".to_string(),
                                        args: vec![Some(a), Some(b)],
                                    };
                                }
                                Self::Other('[')=> {
                                    return Self::Command {
                                        name: "ar".to_string(),
                                        args: vec![None, Some(Self::BracketGroup(Self::parser(it,"Bracket")))],
                                    };
                                }
                                _ => {
                                    panic!("ar匹配错误啦！\n");
                                }
                            }
                        }
                        Some(other_s) => {
                            //这里列举命令，哎我草怎么这么多命令
                            // println!("得到命令:{:?}",other_s);
                            return Self::Symbol(other_s);
                        }
                        None => {
                            if let Some(c) = it.next() {
                                return Self::Symbol(c.to_string());
                            } else {
                                panic!("怎么放了个空的反斜杠!\n");
                            }
                        }
                    }
                }
                ' ' | '\n' => {return Self::Blank;}
                '@' => {
                    let c = Self::get_text(it);
                    return Self::AtModifier(c);
                }
                '%' => {
                    return Self::Comment(Self::get_line(it));
                }
                other => {
                    return Self::Other(other);
                }
            }
        } else {
            panic!("怎么没东西读了!\n");
        }
            
            
    }
    fn must_parser_one(it: &mut std::iter::Peekable<std::str::Chars>) -> Self {
        let w = Self::parser_one(it);
        match &w{
            Self::Blank => {return Self::must_parser_one(it);}
            _ => {return w;}
        }
    }
    fn must_parser_one_brace(it: &mut std::iter::Peekable<std::str::Chars>) -> Self {
        let w = Self::parser_one(it);
        match &w {
            Self::BraceGroup(..) => w,
            Self::Symbol(..) | Self::Other(..) => Self::BraceGroup(vec![w]),
            Self::Blank => {return Self::must_parser_one_brace(it);}
            _ =>{panic!("并不是一个token!");}
        }
    }
    fn try_parser_one_bracket(it: &mut std::iter::Peekable<std::str::Chars>) -> Option<Self> {
        while let Some(c) = it.peek(){
            match c {
                ' '|'\n' => {
                    it.next();
                }
                '[' =>{
                    it.next();
                    return Some(Self::BracketGroup(Self::parser(it, "Bracket")));
                }
                _ =>{
                    return None;
                }
            }
        }
        return None;
    }
    fn must_parser_one_bracket(it: &mut std::iter::Peekable<std::str::Chars>) -> Self {
        let w = Self::try_parser_one_bracket(it);
        match w{
            None =>{panic!("错误，没有识别到Bracket\n");}
            Some(a) => {return a;}
        }
    }

    
    fn parser(it: &mut std::iter::Peekable<std::str::Chars>, tag: &str) -> Vec<Self> {
        // println!("start parser");
        let mut tokens = Vec::new();
        while let Some(c) = it.peek() {
            match c {
                '{' => {
                    it.next();
                    tokens.push(Self::BraceGroup(Self::parser(it, "Brace")));
                }
                '}' => {
                    if tag == "Brace" {
                        it.next();
                        return tokens;
                    } else {
                        panic!("大括号匹配错误")
                    }
                }
                '[' if tag == "Bracket" => {
                    it.next();
                    tokens.push(Self::BracketGroup(Self::parser(it, "Bracket")));
                }
                ']' if tag == "Bracket" => {
                    it.next();
                    return tokens;
                }
                _ => {
                    let w = Self::parser_one(it);
                    
                    if tag.ends_with("_end"){
                        match w {
                            Self::Environment { name , content }if name == tag && content.is_empty()=> {
                                return tokens;
                            }
                            other =>{tokens.push(other)}
                        }
                    }
                    else {
                        tokens.push(w);
                    }
                }
            }
        }
        if tag == "normal" {return tokens;}
        else {
            panic!("没能正确终止!\n");
        }
    }
    fn get_text(it: &mut std::iter::Peekable<std::str::Chars>) -> String{//从{...}读出...
        let mut cnt = 0;
        let mut ans = String::new();
        while let Some(c) = it.next(){
            if (cnt <=0) && (c == ' '|| c =='\n' ){continue;}
            if c == '{' {
                cnt=cnt+1
            }
            else if c == '}'{
                cnt=cnt-1;
            }
            ans.push(c);
            if cnt == 0{
                return ans.trim_start_matches("{").trim_end_matches("}").to_string();
            }
            if cnt < 0 {
                panic!("错误的text读取\n");
            }
        }
        panic!("错误的text读取\n");
    }
    fn normalize(self) -> TexToken{
        match self {
            Self::Environment { name: n, content: c } =>{
                return Self::Environment{name: n, content: Self::vec_normalize(c)};
            }
            Self::Command { name: s, args: t } => {
                return Self::Command{name: s, args: Self::vecop_normalize(t)};
            }
            Self::BraceGroup(t) => Self::BraceGroup(Self::vec_normalize(t)),
            Self::BracketGroup(t) => Self::BracketGroup(Self::vec_normalize(t)),
            Self::SuperScript(t) => Self::SuperScript(Box::new(t.normalize())),
            Self::SubScript(t) => Self::SubScript(Box::new(t.normalize())),
            _ => {return self;}
        }
    }
    fn vecop_normalize(input: Vec<Option<TexToken>> ) -> Vec<Option<TexToken>> {
        let mut ans = Vec::new();
        for i in input{
            match i {
                None => {ans.push(None);}
                Some(w) => {ans.push(Some(w.normalize()))}
            }
        }
        return ans;
    }
    fn vec_normalize(input: Vec<TexToken>) -> Vec<TexToken> {
        let mut ans = Vec::new();
        let mut str = String::new();
        let mut tag =0;//0:empty 1:digit 2:word
        for i in input{
            match i {
                Self::Other(c) => {
                    match c{
                        'a'..='z' | 'A'..='Z' =>{
                            if tag == 0{
                                str.push(c);
                                tag=2;
                            }else if tag == 2{
                                str.push(c);
                            }else if tag == 1{
                                ans.push(Self::Number(str));
                                str=String::new();
                                str.push(c);tag=2;
                            }
                        }
                        '0'..='9' =>{
                            if tag == 0{
                                str.push(c);
                                tag=1;
                            }else if tag == 1{
                                str.push(c);
                            }else if tag == 2{
                                ans.push(Self::Word(str));
                                str=String::new();
                                str.push(c);tag=1;
                            }
                        }
                        _ =>{
                            if tag == 1{
                                ans.push(Self::Number(str));tag=0;
                                str=String::new();
                            }
                            else if tag == 2{
                                ans.push(Self::Word(str));tag=0;
                                str=String::new();
                            }
                            ans.push(Self::Other(c));
                        }
                    }
                }
                Self::Blank =>{
                    if tag == 1{
                        ans.push(Self::Number(str));tag=0;
                        str=String::new();
                    }
                    else if tag == 2{
                        ans.push(Self::Word(str));tag=0;
                        str=String::new();
                    }
                }
                other => {
                    if tag == 1{
                        ans.push(Self::Number(str));tag=0;
                        str=String::new();
                    }
                    else if tag == 2{
                        ans.push(Self::Word(str));tag=0;
                        str=String::new();
                    }
                    ans.push(other.normalize());
                }
            }
        }
        if tag == 1{
            ans.push(Self::Number(str));
        }
        else if tag == 2{
            ans.push(Self::Word(str));
        }
        return ans;
    }
}
pub fn latex_parser(input: &str) -> Vec<TexToken> {
    let mut it = input.chars().peekable();
    let mut ans: Vec<TexToken> = Vec::new();
    // println!("check start!");
    while let Some(_) = it.peek() {
        // println!("start\n");
        ans.push(TexToken::parser_one(&mut it));
        // println!("end\n");
    }
    // println!("check ok!");
    return TexToken::vec_normalize(ans);
    // return ans;
}
