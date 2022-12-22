use crate::post_process::optimise::optimise;
use crate::Args;
use regex::Regex;

fn parse_asm_lines(asm: String) -> Vec<String> {
    let mut output = Vec::new();

    for line in asm.lines() {
        // remove whitespace and comments
        let mut new_line = line.clone();

        let mut line_split = new_line.clone().split(";");
        // detect comments which start with ';-' which should be kept
        if line_split.nth(1).is_some() {
            new_line =
                new_line.clone().split(";").nth(0).unwrap();
        }
        new_line = new_line.trim();

        if new_line == "" {
            continue;
        }

        output.push(new_line.to_string());
    }

    output
}

pub fn post_process(asm: String, args: &Args) -> String {
    let mut output = String::new();

    let mut indent = 4;

    let indent_re: Regex = Regex::new(":$").unwrap();

    for line in optimise(parse_asm_lines(asm), args) {
        let should_unindent =
            indent_re.is_match(line.as_str());

        if should_unindent {
            indent -= 4;
            // new line before labels and sections
            //output += "\n";
        }

        output += &(" ".repeat(indent).to_owned()
            + line.as_str()
            + "\n");

        if should_unindent {
            indent += 4;
        }
    }

    output
}
