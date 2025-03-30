use pulldown_cmark::{Parser, html};
use std::io::{self, Write};

fn main() {
    println!("Enter Markdown text (Ctrl+D or Ctrl+Z to finish):");

    let mut markdown_input = String::new();
    
    loop {
        let mut line = String::new();
        io::stdin().read_line(&mut line).unwrap();
        
        if line.trim().is_empty() {
            break;
        }

        markdown_input.push_str(&line);
    }

    let parser = Parser::new(&markdown_input);
    let mut html_output = String::new();
    html::push_html(&mut html_output, parser);

    println!("\nConverted HTML:\n{}", html_output);
}
