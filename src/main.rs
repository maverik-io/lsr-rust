use std::env;
use std::fs;

fn main() {
    let terminal_size = termsize::get().unwrap();
    let current_dir = env::current_dir().unwrap();
    let args: Vec<String> = env::args().collect();
    println!("{}", "-=".repeat(terminal_size.cols as usize / 2));
    print_dir_contents(&current_dir, 0, &args);
    println!("{}", "-=".repeat(terminal_size.cols as usize / 2));
}

fn print_dir_contents(dir: &std::path::Path, indent: u32, args: &Vec<String>) {
    for file in fs::read_dir(dir).unwrap() {
        let mut icon = String::from("    ");

        let file = file.unwrap();

        match file.file_name().into_string().unwrap().split(".").last() {
            Some("rs") => icon = String::from("  󱘗  "),
            Some("md") => icon = String::from("    "),
            Some("sh") => icon = String::from("    "),
            Some("json") => icon = String::from("    "),
            Some("yaml") => icon = String::from("    "),
            Some("toml") => icon = String::from("    "),
            Some("yml") => icon = String::from("    "),
            Some("lock") => icon = String::from("    "),
            Some("py") => icon = String::from("  󰌠  "),
            Some("pyc") => icon = String::from("  󰌠  "),
            Some("html") => icon = String::from("    "),
            Some("css") => icon = String::from("    "),
            Some("js") => icon = String::from("    "),
            Some("ts") => icon = String::from("    "),
            Some("jsx") => icon = String::from("    "),
            Some("tsx") => icon = String::from("    "),
            Some("java") => icon = String::from("    "),
            Some("c") => icon = String::from("    "),
            Some("cpp") => icon = String::from("    "),
            Some("h") => icon = String::from("    "),
            Some("hpp") => icon = String::from("    "),
            Some("go") => icon = String::from("  󰟓  "),
            Some("rb") => icon = String::from("    "),
            Some("php") => icon = String::from("    "),
            Some("lua") => icon = String::from("    "),
            Some("sql") => icon = String::from("    "),
            Some("swift") => icon = String::from("    "),
            Some("psd") => icon = String::from("    "),
            Some("ai") => icon = String::from("    "),
            Some("pdf") => icon = String::from("  󰈦  "),
            Some("png") => icon = String::from("    "),
            Some("jpg") => icon = String::from("    "),
            Some("jpeg") => icon = String::from("    "),
            Some("gif") => icon = String::from("    "),
            Some("svg") => icon = String::from("    "),
            Some("webp") => icon = String::from("    "),
            Some("mp3") => icon = String::from("  󰝚  "),
            Some("wav") => icon = String::from("  󰝚  "),
            Some("flac") => icon = String::from("  󰝚  "),
            Some("ogg") => icon = String::from("  󰝚  "),
            Some("m4a") => icon = String::from("  󰝚  "),
            Some("opus") => icon = String::from("  󰝚  "),
            


            Some(&_) => (),
            None => (),
        }

        if file.file_type().unwrap().is_dir() {
            icon = String::from("   ")
        }

        let out = file.file_name().into_string().unwrap();
        let out = out.split("/").last().unwrap();
        let indent_str = " ▏".repeat(indent as usize);
        if !out.starts_with(".") || args.contains(&String::from("-a")) {
            println!("{}{}{}", indent_str, icon, out);

            if file.file_type().unwrap().is_dir() {
                let next_indent = indent + 1;
                print_dir_contents(&file.path(), next_indent, args);
            }
        }
    }
}
