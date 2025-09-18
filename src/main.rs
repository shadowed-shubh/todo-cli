use ncurses::*;
use std::cmp::min;

const REG_PAIR: i16 = 0;
const HIG_PAIR: i16 = 1;

fn main() {
    initscr(); // initialize ncurses
    noecho();
    //let hig = getmaxy(stdscr());
    //let wid = getmaxx(stdscr());

    //lists and other stuff
    let mut todos: Vec<String> = vec![
        "todo1".to_string(),
        "todo2".to_string(),
        "todo3".to_string(),
        "hi".to_string(),
    ];
    let mut stat = [false, false, false, false];

    //ui comps
    start_color();
    init_pair(REG_PAIR, COLOR_WHITE, COLOR_BLACK);
    init_pair(HIG_PAIR, COLOR_BLACK, COLOR_WHITE);

    let done: &str = "[x]";
    let undone: &str = "[ ]";

    //quting mechanism
    let mut quit: bool = false;

    let mut curr = 0;

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
            'a' => {
                let sen: String = "hello".to_string();
                add_todo(&mut todos, sen);
            }
            //a => add todo
            //d => delete todo
            //e => save or persist
            //\t => edit todo
            //test
            _ => {}
        }
    }

    refresh();
    endwin();
}

fn add_todo(v: &mut Vec<String>, sen: String) {
    v.push(sen.to_string());
}
