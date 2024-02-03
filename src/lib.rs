pub mod que;
pub mod ui;
pub mod update;

use rand::Rng;
use std::collections::HashMap;
use std::iter::zip;

pub struct App {
    mapping: HashMap<u8, u8>,
    words: HashMap<u16, (String, String)>,
    layout_left: [[u8; 5]; 3],
    layout_right: [[u8; 5]; 3],
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
        let default_layout: String = match std::fs::read_to_string(format!("layouts/default.conf"))
        {
            Ok(layout) => layout,
            Err(_) => return Err("Default Layout Not Set"),
        };

        let mut hmap: HashMap<u8, u8> = HashMap::with_capacity(30);
        let mut layout_left: [[u8; 5]; 3] = [[32; 5]; 3];
        let mut layout_right: [[u8; 5]; 3] = [[32; 5]; 3];

        for (i, (dl, tl)) in zip(default_layout.lines(), target_layout.lines()).enumerate() {
            for (j, (ds, ts)) in zip(dl.split_whitespace(), tl.split_whitespace()).enumerate() {
                let ds = ds.to_lowercase();
                let ts = ts.to_lowercase();
                if j <= 4 {
                    layout_left[i][j] = ts.parse::<char>().unwrap() as u8;
                } else {
                    layout_right[i][j - 5] = ts.parse::<char>().unwrap() as u8;
                }
                hmap.insert(
                    ds.parse::<char>().unwrap() as u8,
                    ts.parse::<char>().unwrap() as u8,
                );
            }
        }

        if hmap.len() != 30 {
            return Err("Layout Not Set Properly");
        }

        let mut app = App {
            mapping: hmap,
            words: HashMap::with_capacity(1000),
            layout_left,
            layout_right,
        };
        app.generate()?;
        Ok(app)
    }

    fn generate(&mut self) -> Result<(), &'static str> {
        let wordlist = std::fs::read_to_string("wordlist.txt");
        if let Err(_) = wordlist {
            return Err("Wordlist Not Found");
        } else {
            let wordlist = wordlist.unwrap();
            let mut s = wordlist.lines();
            for i in 0..999 {
                let s1 = s.next().unwrap().to_owned();
                let s2 = self.map_to_target(&s1);
                self.words.insert(i, (s1, s2));
            }
            Ok(())
        }
    }

    #[inline]
    fn map_to_target(&self, s: &str) -> String {
        let mut new_s = String::with_capacity(s.len());
        for c in s.chars() {
            let k = self.mapping.get(&(c as u8));
            new_s.push(k.unwrap().to_owned() as char);
        }
        new_s
    }

    #[inline]
    pub fn get_pair(&self) -> &(String, String) {
        let mut rng = rand::thread_rng();
        let res = self.words.get(&(rng.gen::<u16>() % 1000)).unwrap();
        res
    }
}
