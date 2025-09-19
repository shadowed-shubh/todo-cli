use ncurses::*;
use std::cmp::min;
use std::fs::File;
use std::io::{BufRead, Write};

const REG_PAIR: i16 = 0;
const HIG_PAIR: i16 = 1;

fn save_state(file_path: &str, todos: &Vec<String>, stat: &Vec<bool>) {
    let mut file = File::create(file_path).unwrap();
    for (todo, sta) in todos.iter().zip(stat.iter()) {
        writeln!(file, "{}:{}", sta, todo).unwrap();
    }
}
fn load_state(file_path: &str, todos: &mut Vec<String>, stat: &mut Vec<bool>) {
    {
        let file = File::open(file_path).unwrap();
        for line in std::io::BufReader::new(file).lines() {
            match parse_todo(&line.unwrap()) {
                Some((st, title)) => {
                    todos.push(title.to_string());
                    stat.push(st);
                }
                None => {
                    eprintln!("ERROR: not a vaild format");
                    std::process::exit(1);
                }
            }
        }
    }
}

fn parse_todo(line: &str) -> Option<(bool, &str)> {
    let done_pre = "true:";
    let undone_pre = "false:";

    if line.starts_with(done_pre) {
        return Some((true, &line[done_pre.len()..]));
    }
    if line.starts_with(undone_pre) {
        return Some((false, &line[undone_pre.len()..]));
    } else {
        None
    }
}

fn main() {
    //add args to read filepath and file missing
    let mut args = std::env::args();
    args.next().unwrap();

    let file_path = match args.next() {
        Some(file_path) => file_path,
        None => {
            eprintln!("Usage: todo-rs <file_path>");
            eprintln!("ERROR: no file path is provided");
            std::process::exit(1);
        }
    };
    //lists and other stuff
    let mut todos = Vec::<String>::new();

    let mut stat = Vec::<bool>::new();

    //ui comps

    let done: &str = "[x]";
    let undone: &str = "[ ]";

    let mut curr = 0;

    load_state(&file_path, &mut todos, &mut stat);
    initscr(); // initialize ncurses
    noecho();

    //let hig = getmaxy(stdscr());
    //let wid = getmaxx(stdscr());

    start_color();
    init_pair(REG_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIG_PAIR, COLOR_BLACK, COLOR_WHITE);

    //quting mechanism
    let mut quit: bool = false;
    while !quit {
        for (row, todo) in todos.iter().enumerate() {
            let pair: i16 = {
                if row == curr {
                    HIG_PAIR
                } else {
                    REG_PAIR
                }
            };

            attron(COLOR_PAIR(pair));

            let stat_sy = if stat[row] { done } else { undone };

            mv(row as i32, 0);
            addstr(stat_sy);
            mv(row as i32, 3);
            addstr(todo);

            attroff(COLOR_PAIR(pair));
        }
        refresh();
        let key = getch();
        match key as u8 as char {
            'q' => quit = true,
            'w' => {
                curr = curr.saturating_sub(1);
            }

            's' => curr = min(1 + curr, todos.len() - 1),
            '\n' => {
                stat[curr] = !stat[curr];
            }
            //'a' => {
            //    let sen: String = "hello".to_string();
            //    add_todo(&mut todos, sen);
            //}
            //a => add todo
            //d => delete todo
            //e => save or persist
            //\t => edit todo
            _ => {}
        }
    }

    save_state(&file_path, &todos, &stat);
    refresh();
    endwin();
}
