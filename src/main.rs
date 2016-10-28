extern crate ncurses;

use std::env;
use std::path::Path;
use std::fs::File;
use std::io::{Read, Write};
use std::collections::HashMap;


fn usage() {
	println!("Usage: emojy [TRANSLATION_FILE] EMOJY_FILE");
}

fn load_file(p: &str) -> Option<String> {

	let f_ = File::open(&Path::new(p));

    match f_ {
        Err(e) => { 
        	println!("{:?}", e);
        	None
        },

        Ok(mut f) => {
            let mut s = String::new();
            let res = f.read_to_string(&mut s);

            match res {
                Err(e) => {
                	println!("{:?}", e);
                	None
                },
                Ok(_) => Some(s),
            }
        }
    }
}

fn load_translation(p: &str) -> Option<HashMap<String,String>> {

	match load_file(p) {
        None => None,

        Some(s) => {
            let mut t = HashMap::new();
            for line in s.lines() {
            	let source_char = line.chars().nth(0);
            	let target_char = line.chars().last();
            	if source_char.is_some() && target_char.is_some() {
            		let source_char = source_char.unwrap();
            		let target_char = target_char.unwrap();
            		t.insert(source_char.to_string(), target_char.to_string());
            	}
            }
            Some(t)
        }
    }
}

// translate source using the translation hashmap, silently ignoring all non translatable chars
fn translate(source: &str, translation: HashMap<String,String>) -> String {

	let mut translated = String::new();
	
	for c in source.chars() {
		match translation.get(&c.to_string()) {
			None => {}
			Some(t) => {
				translated.push_str(t);

			}
		}
	}

	translated
}

fn build_bracemap(code: &str) -> Option<HashMap<usize,usize>> {
	let mut b_tmp: Vec<usize> = Vec::new();
	let mut b = HashMap::new();

	for (p,c) in code.chars().enumerate() {
		match c {
			'[' => b_tmp.push(p),
			']' => {
				match b_tmp.pop() {
					None => return None,
					Some(s) => {
						b.insert(p,s);
						b.insert(s,p);
					}
				}
			},
			_ => ()
		}
	}

	Some(b)

}

fn run_brainfuck(code: &str) {
	ncurses::initscr();

	ncurses::noecho();

	

	let mut v: Vec<u8> = Vec::new();
	let mut cp: usize = 0;
	let mut sp: usize = 0;
	let bracemap;

	match build_bracemap(code) {
		Some(b) => bracemap = b,
		None => {
			println!("Syntax Error: Couldn't build bracemap.");
			return
		}
	}

	while sp < code.chars().count() {
		let c = code.chars().nth(sp).unwrap();


		if cp >= v.len() {
			v.resize(cp+1, 0);
		}
		
		match c {
			'>' => cp += 1,
			'<' => cp -= 1,
			'+' => v[cp] = if v[cp] < 255 { v[cp] + 1 } else { 0 },
			'-' => v[cp] = if v[cp] > 0 { v[cp] - 1 } else { 255 },
			'[' => {
				if v[cp] == 0 {
					sp = bracemap[&sp]
				}
			},
			']' => {
				if v[cp] != 0 {
					sp = bracemap[&sp]
				}
			},
			'.' => { 
				match std::str::from_utf8(&[v[cp]]) {
					Err(_) => {},
					Ok(c) => { ncurses::printw(c); }
				}
			},
			',' => {
				v[cp] = ncurses::getch() as u8;
			}
			_ => ()
		}

		sp += 1;
	}

	ncurses::printw("\n\nPress enter to exit\n\n");

	ncurses::getch();

	ncurses::endwin();

	()
}

fn main() {
    let args: Vec<String> = env::args().collect();

    let source_file;

    let mut translation_file = "translation";

    match args.len() {
        2 => source_file = args.get(1).unwrap(),
        3 => {
        	translation_file = args.get(1).unwrap();
        	source_file = args.get(2).unwrap();
        },
        _ => return usage()
    }

    let emojy_code;

    match load_file(source_file) {
    	None => {
    		println!("Couldn't load source file");
    		return usage()
    	},
    	Some(s) => emojy_code = s,
    }

    let translation;

    match load_translation(translation_file) {
    	None => {
    		println!("Couldn't load translation file");
    		return usage();
    	},
    	Some(t) => translation = t
    }

    let brainfuck_code = translate(&emojy_code, translation);

    run_brainfuck(&brainfuck_code);
}
