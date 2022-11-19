use regex::Regex;

pub(crate) fn  post_process(asm: String) -> String{
    let mut output = String::new();

    let mut indent = 4;

    let indent_re: Regex = Regex::new("^(section ?.[a-zA-Z0-9]+)|([a-zA-Z0-9_-]+:)$").unwrap();

    for line in asm.lines() {
        // remove whitespace
        let new_line = line.split(";").nth(0).unwrap_or("").trim();

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
