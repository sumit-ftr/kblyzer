pub mod ui;
pub mod update;
use rand::Rng;
use std::collections::HashMap;

#[derive(Debug)]
pub struct App {
    mapping: HashMap<u8, u8>,
    words: HashMap<u16, (String, String)>,
    quit: bool,
}

impl App {
    pub fn new(mut a: std::env::Args) -> Result<App, &'static str> {
        a.next();
        let target_layout: String = match a.next() {
            Some(layout_name) => layout_name,
            None => return Err("Insufficient Arguments"),
        };
        let target_layout: String =
            match std::fs::read_to_string(format!("layouts/{target_layout}")) {
                Ok(layout) => layout,
                Err(_) => return Err("Layout Not Found"),
            };
        let default_layout: String = match std::fs::read_to_string(format!("layouts/.default")) {
            Ok(layout) => layout,
            Err(_) => return Err("Default Layout Not Set"),
        };

        let mut hmap: HashMap<u8, u8> = HashMap::with_capacity(30);

        for (dl, tl) in std::iter::zip(default_layout.lines(), target_layout.lines()) {
            for (ds, ts) in std::iter::zip(dl.split_whitespace(), tl.split_whitespace()) {
                hmap.insert(
                    ds.parse::<char>().unwrap() as u8,
                    ts.parse::<char>().unwrap() as u8,
                );
            }
        }

        let mut app = App {
            mapping: hmap,
            words: HashMap::with_capacity(1000),
            quit: false,
        };
        app.shuffle();
        Ok(app)
    }

    fn shuffle(&mut self) {
        let wordlist = std::fs::read_to_string("wordlist.txt").unwrap_or_else(|_| {
            self.quit();
            "".to_string()
        });

        let mut s = wordlist.lines();
        for i in 0..999 {
            let s1 = s.next().unwrap();
            // removing the new line character
            let s1 = s1[..s1.len() - 1].to_owned();
            let s2 = self.map_to_target(&s1);
            self.words.insert(i, (s1, s2));
        }
    }

    fn map_to_target(&self, s: &str) -> String {
        let mut new_s = String::with_capacity(s.len());
        for c in s.chars() {
            let k = self.mapping.get(&(c as u8));
            new_s.push(k.unwrap().to_owned() as char);
        }
        new_s
    }

    pub fn quit(&mut self) {
        self.quit = true;
    }

    pub fn should_quit(&self) -> bool {
        self.quit
    }

    pub fn get_word(&self) -> &(String, String) {
        let mut rng = rand::thread_rng();
        let res = self.words.get(&(rng.gen::<u16>() % 1000)).unwrap();
        res
    }
}
