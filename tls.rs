use std::{env, fs, io};

const ROOM: bool = true;

fn main() -> io::Result<()> {
  let args: Vec<String> = env::args().collect();
  if args.len() > 1 && (args[1] == "-h" || args[1] == "--help") { help(); }
  let path = args.get(1).map_or_else(|| ".".into(), |s| s.clone());

  let mut entries: Vec<_> = fs::read_dir(&path)?
    .filter_map(Result::ok)
    .collect();

  entries.sort_by(|a, b| {
    a.file_type().map_or(false, |ft| ft.is_dir()).cmp(&b.file_type().map_or(false, |ft| ft.is_dir())).reverse()
    .then_with(|| a.file_name().to_ascii_lowercase().cmp(&b.file_name().to_ascii_lowercase()))
  });
  
  if ROOM == false {
    let mut left = false;
    for entry in entries {
      let color = if entry.path().is_dir() { "\x1b[1;34m" } else { "\x1b[1;32m" };
      if left { print!("| "); }
      print!("{}{}{} ", color, entry.file_name().to_string_lossy(), "\x1b[0m");
      left = true; 
    }
    println!();
  }
  else {
    for entry in entries {
      let color = if entry.path().is_dir() { "\x1b[1;34m" } else { "\x1b[1;32m" };
      println!("{}{}{}", color, entry.file_name().to_string_lossy(), "\x1b[0m");
    }
  }
  Ok(())
}

fn help() {
  let text = format!("\n\
    \x1b[1;37m\
    \tUsage: ls [DIR]\n\
    \tLists current directories and files when no args are passed.\n\
    \t\n\
    \tDirectories are \x1b[1;34mBLUE\x1b[1;37m\n\
    \tFiles are \x1b[1;32mGREEN\x1b[0m\n\
  ");
  println!("{}", text);
  std::process::exit(0);
}
