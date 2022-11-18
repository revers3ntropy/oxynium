use regex::Regex;

pub(crate) fn  post_process(asm: String) -> String{
    let mut output = String::new();

    let mut indent = 4;

    let indent_re: Regex = Regex::new("^(section ?.[a-zA-Z]+)|([a-zA-Z_-]+:)$").unwrap();

    for line in asm.lines() {
        // remove whitespace
        let new_line = line.trim();

        if new_line == "" {
            continue;
        }

        let should_unindent = indent_re.is_match(new_line);

        if should_unindent {
            indent -= 4;
            output += "\n";
        }

        output += &(" ".repeat(indent).to_owned() + new_line + "\n");

        if should_unindent {
            indent += 4;
        }
    }

    output
}
