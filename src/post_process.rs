pub(crate) fn  post_process(asm: String) -> String{
    let mut output = String::new();

    for line in asm.lines() {
        // remove whitespace
        let line = line.trim().to_owned();

        if line == "" {
            continue;
        }

        output += &(line + &*"\n".to_owned());
    }

    output
}
