use clap::Parser;
use crossterm::{cursor::MoveUp, terminal::ClearType};
use crossterm::{terminal::Clear, ExecutableCommand, Result};
use std::collections::VecDeque;
use std::io::{stdin, stdout, BufRead};

#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Number of lines to display at a time
    #[clap(short = 'l', long = "lines", value_parser, default_value_t = 5)]
    lines: usize,
}

fn main() -> Result<()> {
    let args = Args::parse();
    let lines_to_print = args.lines;
    let mut deque: VecDeque<String> = VecDeque::with_capacity(lines_to_print);
    let mut moves = 0;
    loop {
        let mut buffer = String::new();
        let bytes = stdin().lock().read_line(&mut buffer)?;
        if bytes == 0 {
            break;
        }
        if deque.len() == lines_to_print {
            deque.pop_front();
        }
        deque.push_back(buffer.trim_end_matches('\n').to_string());
        if moves > 0 {
            stdout().execute(MoveUp(moves))?; //0 can move cursor 1 up
        }
        for line in deque.iter() {
            if moves > 0 {
                stdout().execute(Clear(ClearType::CurrentLine))?;
            }
            println!("{}", line);
        }
        moves = deque.len() as u16;
    }
    Ok(())
}
