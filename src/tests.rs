#![cfg(test)]
use crate::to_markdown::*;
use std::fs;
use std::io::Write;

#[test]
fn test0() -> std::io::Result<()> {
    let content = fs::read_to_string("input_dir/input0.md")?;
    let new_article = parse_markdown(&content);
    let mut file = fs::File::create("output_dir/input0.md")?;
    writeln!(file, "{}", new_article)?;
    Ok(())
}
#[test]
fn test1() -> std::io::Result<()> {
    let content = fs::read_to_string("input_dir/input1.md")?;
    let new_article = parse_markdown(&content);
    let mut file = fs::File::create("output_dir/input1.md")?;
    writeln!(file, "{}", new_article)?;
    Ok(())
}
#[test]
fn test2() -> std::io::Result<()> {
    let content = fs::read_to_string("input_dir/input2.md")?;
    let new_article = parse_markdown(&content);
    let mut file = fs::File::create("output_dir/input2.md")?;
    writeln!(file, "{}", new_article)?;
    Ok(())
}
#[test]
fn test3() -> std::io::Result<()> {
    let content = fs::read_to_string("input_dir/input3.md")?;
    let new_article = parse_markdown(&content);
    let mut file = fs::File::create("output_dir/input3.md")?;
    writeln!(file, "{}", new_article)?;
    Ok(())
}

#[test]
fn test4() -> std::io::Result<()> {
    let content = fs::read_to_string("input_dir/input4.md")?;
    let new_article = parse_markdown(&content);
    let mut file = fs::File::create("output_dir/input4.md")?;
    writeln!(file, "{}", new_article)?;
    Ok(())
}


#[test]
fn example1() -> std::io::Result<()> {
    let content = fs::read_to_string("input_dir/example1.md")?;
    let new_article = parse_markdown(&content);
    let mut file = fs::File::create("output_dir/example1.md")?;
    writeln!(file, "{}", new_article)?;
    Ok(())
}
#[test]
fn example2() -> std::io::Result<()> {
    let content = fs::read_to_string("input_dir/example2.md")?;
    let new_article = parse_markdown(&content);
    let mut file = fs::File::create("output_dir/example2.md")?;
    writeln!(file, "{}", new_article)?;
    Ok(())
}
#[test]
fn example3() -> std::io::Result<()> {
    let content = fs::read_to_string("input_dir/example3.md")?;
    let new_article = parse_markdown(&content);
    let mut file = fs::File::create("output_dir/example3.md")?;
    writeln!(file, "{}", new_article)?;
    Ok(())
}