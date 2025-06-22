use crate::latexparser::TexToken;
use crate::latexparser::latex_parser;

#[derive(Debug)]
pub struct FrontMatters {
    title: Option<String>,
    categories: Option<String>,
    tags: Option<Vec<String>>,
    mathjax: bool,
    password: Option<String>,
    top: bool,
}
impl FrontMatters {
    pub fn new(s: &str) -> Self {
        let mut lines = s.lines().peekable();
        let mut title: Option<String> = None;
        let mut password: Option<String> = None;
        let mut categories: Option<String> = None;
        let mut mathjax = false;
        let mut top = false;
        let mut tags: Option<Vec<String>> = None;
        while let Some(line) = lines.next() {
            if line.starts_with("title:") {
                title = Some(line.trim_start_matches("title:").trim().to_string());
            } else if line.starts_with("password:") {
                password = Some(line.trim_start_matches("password:").trim().to_string());
            } else if line.starts_with("categories:") {
                categories = Some(line.trim_start_matches("categories:").trim().to_string());
            } else if line.starts_with("mathjax:") {
                match line.trim_start_matches("mathjax:").trim() {
                    "true" => {
                        mathjax = true;
                    }
                    "false" => {
                        mathjax = false;
                    }
                    _ => {
                        panic!("不合法的mathjax声明");
                    }
                }
            } else if line.starts_with("top:") {
                match line.trim_start_matches("top:").trim() {
                    "true" => {
                        top = true;
                    }
                    "false" => {
                        top = false;
                    }
                    _ => {
                        panic!("不合法的top声明");
                    }
                }
            } else if line.starts_with("tags:") {
                let otags = line.trim_start_matches("tags:").trim();
                let nowtags = otags.trim_start_matches("[").trim_end_matches("]").trim();
                tags = Some(nowtags.split(',').map(|s| s.trim().to_string()).collect());
            }
        }
        return FrontMatters {
            title,
            categories,
            tags,
            mathjax,
            password,
            top,
        };
    }
}
impl ToString for FrontMatters {
    fn to_string(&self) -> String {
        let mut ans = String::new();
        ans.push_str("---\n");
        if let Some(ref t) = self.title {
            ans.push_str("title: ");
            ans.push_str(t);
            ans.push('\n');
        }
        if let Some(ref c) = self.categories {
            ans.push_str("categories: ");
            ans.push_str(c);
            ans.push('\n');
        }
        if let Some(ref p) = self.password {
            ans.push_str("password: ");
            ans.push_str(p);
            ans.push('\n');
        }
        if let Some(ref t) = self.tags {
            let mut stags = String::new();
            stags.push_str("tags: [");
            for tag in t {
                stags.push_str(tag);
                stags.push(',');
            }
            ans.push_str(stags.trim_end_matches(","));
            ans.push(']');
            ans.push('\n');
        }
        if self.mathjax {
            ans.push_str("mathjax: true\n");
        }
        if self.top {
            ans.push_str("top: true\n");
        }
        ans.push_str("---");
        return ans;
    }
}
#[derive(Debug)]
pub enum HtmlSetting {
    Toc,
    More,
}
impl ToString for HtmlSetting {
    fn to_string(&self) -> String {
        // let mut ans = String::new();
        match self {
            HtmlSetting::Toc => ("<!-- toc -->").to_string(),
            HtmlSetting::More => ("<!-- more -->").to_string(),
        }
        // return ans;
    }
}
#[derive(Debug)]
pub struct Heading {
    level: usize,
    text: String,
}
impl ToString for Heading {
    fn to_string(&self) -> String {
        let mut ans = String::new();
        for _ in 0..(self.level) {
            ans.push('#');
        }
        ans.push(' ');
        ans.push_str(&self.text);
        return ans;
    }
}
#[derive(Debug)]
pub enum Token {
    Front(FrontMatters),
    Html(HtmlSetting),
    Head(Heading),
    Document(String),
    DisplayMath(Vec<TexToken>),
    InlineMath(Vec<TexToken>),
    Quotation(String),
    CodeBlock(String, String),
    Blank,
}
impl ToString for Token {
    fn to_string(&self) -> String {
        match self {
            Token::Front(f) => f.to_string() + "\n",
            Token::Html(h) => h.to_string() + "\n",
            Token::Head(h) => h.to_string() + "\n",
            Token::Document(d) => d.to_string(),
            Token::DisplayMath(d) => ("$$\n").to_string() + TexToken::vectostring(d,true).trim() + ("\n$$"),
            Token::InlineMath(d) => ("$").to_string() + TexToken::vectostring(d,false).trim() + ("$"),
            Token::Blank => ("\n\n").to_string(),
            Token::Quotation(s) => {
                let mut ans = String::new();
                let mut it = s.lines();
                while let Some(ws) = it.next() {
                    ans.push_str("> ");
                    ans.push_str(ws);
                    ans.push('\n');
                }
                return ans;
            }
            Token::CodeBlock(lang, codes) => {
                ("```").to_string() + lang + ("\n") + codes + ("```\n")
            }
        }
    }
}
pub fn parse(input: &str) -> Vec<Token> {
    // println!("now input is:{:?}",input);
    let mut tokens: Vec<Token> = Vec::new();
    let mut lines = input.lines().peekable();

    if let Some(pol) = lines.peek() {
        let pline = pol.trim();
        if pline == "---" {
            lines.next();
            let mut front = String::new();
            while let Some(ol) = lines.next() {
                let line = ol.trim();
                // println!("now check:{:?}",line);
                if line == "---" {
                    lines.next();
                    break;
                }
                front.push_str(line);
                front.push('\n');
            }
            tokens.push(Token::Front(FrontMatters::new(front.trim())));
        }
    }
    while let Some(ol) = lines.next() {
        let line = ol.trim();
        if line.is_empty() {
            // tokens.push(Token::Blank);
            continue;
        }
        if line.starts_with("<!--") {
            let tag = line
                .trim()
                .trim_start_matches("<!--")
                .trim_end_matches("-->")
                .trim();
            if tag == "toc" {
                tokens.push(Token::Html(HtmlSetting::Toc));
            } else if tag == "more" {
                tokens.push(Token::Html(HtmlSetting::More));
            }
            continue;
        }
        if line.starts_with("#") {
            let level = line.chars().take_while(|&c| c == '#').count();
            if level > 6 {
                panic!("标题不应该超过六级!\n");
            } else {
                let text = line
                    .trim_start_matches(&("#".repeat(level)))
                    .trim()
                    .to_string();
                // println!("now check:{:?}\n",text);
                tokens.push(Token::Head(Heading { level, text }));
            }
            continue;
        }
        if line == "$$" {
            let mut mathstr = String::new();
            while let Some(pol) = lines.next() {
                let pline = pol.trim();
                if pline == "$$" {
                    break;
                } else {
                    mathstr.push_str(pline);
                    mathstr.push('\n');
                }
            }
            tokens.push(Token::DisplayMath(latex_parser(mathstr.trim())));
            tokens.push(Token::Blank);
            continue;
        }
        if line.starts_with(">") {
            let mut quotestr = String::new();
            quotestr.push_str(line.trim_start_matches(">").trim());
            quotestr.push('\n');
            while let Some(pol) = lines.peek() {
                let pline = pol.trim();
                if pline.starts_with(">") {
                    quotestr.push_str(pline.trim_start_matches(">").trim());
                    quotestr.push('\n');
                    lines.next();
                } else {
                    break;
                }
            }
            tokens.push(Token::Quotation(quotestr));
            tokens.push(Token::Blank);
            continue;
        }
        if line.starts_with("```") {
            let mut codestr = String::new();
            let lang = line.trim_start_matches("```").trim();
            while let Some(pol) = lines.next() {
                let pline = pol.trim();
                if pline.starts_with("```") {
                    break;
                }
                codestr.push_str(pline);
                codestr.push('\n');
            }
            tokens.push(Token::CodeBlock(lang.to_string(), codestr));
            continue;
        }
        let mut is_inline: bool = false;
        let mut mathstr = String::new();
        let mut docustr = String::new();
        let mut it_line = line.chars().peekable();
        while let Some(c)= it_line.next() {
            if c == '$' {
                if it_line.peek() == Some(&'$'){
                    it_line.next();
                }
                if is_inline {
                    is_inline = false;
                    tokens.push(Token::InlineMath(latex_parser(&mathstr)));
                    mathstr = String::new();
                } else {
                    is_inline = true;
                    tokens.push(Token::Document(docustr));
                    docustr = String::new();
                }
            } else {
                if is_inline {
                    mathstr.push(c);
                } else {
                    docustr.push(c);
                }
            }
        }
        if !docustr.is_empty() {
            tokens.push(Token::Document(docustr));
        }
        tokens.push(Token::Blank);
    }

    return tokens;
}

