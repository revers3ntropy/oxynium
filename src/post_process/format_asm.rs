use regex::Regex;

pub fn post_process(asm: String) -> String {
    let mut output = String::new();

    let mut indent = 4;

    let indent_re: Regex = Regex::new("^(section ?.)|([a-zA-Z0-9_-]+:)$").unwrap();

    for line in asm.lines() {
        // remove whitespace and comments
        let mut new_line = line.clone();

        let mut line_split = new_line.clone().split(";");
        // detect comments which start with ';-' which should be kept
        if let Some(comment) = line_split.nth(1) {
            if comment.chars().nth(0).unwrap_or(' ') != '-' {
                new_line = new_line.clone().split(";").nth(0).unwrap();
            }
        }
        new_line = new_line.trim();

        if new_line == "" {
            continue;
        }

        let should_unindent = indent_re.is_match(new_line);

        if should_unindent {
            indent -= 4;
            // new line before labels and sections
            //output += "\n";
        }

        output += &(" ".repeat(indent).to_owned() + new_line + "\n");

        if should_unindent {
            indent += 4;
        }
    }

    output
}
