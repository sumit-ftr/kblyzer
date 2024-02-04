use crate::App;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Que<'a> {
    qd: VecDeque<&'a String>,
    qt: VecDeque<&'a String>,
    line: [u16; 3],
    pub dline: [String; 3],
    pub tline: [String; 3],
    line_len: u16,
    quit: bool,
}

impl<'a> Que<'a> {
    pub fn new(app: &'a App, frame_size: u16) -> Que<'a> {
        let x = Que::set_line_len(frame_size) as usize;
        let y = [
            String::with_capacity(x + 1),
            String::with_capacity(x + 1),
            String::with_capacity(x + 1),
        ];
        let mut nq = Que {
            qd: VecDeque::with_capacity(128),
            qt: VecDeque::with_capacity(128),
            line: [0; 3],
            dline: y.clone(),
            tline: y,
            line_len: x as u16,
            quit: false,
        };
        for _ in nq.qd.len()..nq.qd.capacity() {
            let (a, b) = app.get_pair();
            nq.qd.push_back(a);
            nq.qt.push_back(b);
        }
        nq.set_line();
        nq
    }

    #[inline]
    fn set_line_len(frame_size: u16) -> u16 {
        frame_size
    }

    #[inline]
    pub fn get_line_len(&self) -> u16 {
        self.line_len
    }

    #[inline]
    fn set_line(&mut self) {
        let mut index = 0usize;
        for (n, i) in self.line.iter_mut().enumerate() {
            let mut cur_line_len = 0u16;
            for j in index..self.qd.capacity() {
                if cur_line_len + self.qd[j].len() as u16 <= self.line_len {
                    cur_line_len += self.qd[j].len() as u16 + 1;
                    self.dline[n].push_str(self.qd[j]);
                    self.dline[n].push(' ');
                    self.tline[n].push_str(self.qt[j]);
                    self.tline[n].push(' ');
                } else {
                    index = j;
                    break;
                }
            }
            *i = index as u16;
        }
    }

    pub fn sync_on_nspace(&mut self, app: &'a App) {
        // syncing word queue(q) in Que
        for _ in 0..self.line[0] {
            self.qd.pop_front();
            self.qt.pop_front();
            let (a, b) = app.get_pair();
            self.qd.push_back(a);
            self.qt.push_back(b);
        }
        // syncing line indexes and strings
        for i in 0..self.line.len() - 1 {
            self.line[i] = self.line[i + 1] - self.line[i];
            self.dline[i] = self.dline[i + 1].to_owned();
            self.tline[i] = self.tline[i + 1].to_owned();
        }
        // syncing last line index and string
        self.dline[self.dline.len() - 1].clear();
        self.tline[self.tline.len() - 1].clear();
        let mut cur_line_len = 0;
        for j in self.line[self.line.len() - 2] as usize..self.qd.capacity() {
            if cur_line_len + self.qd[j].len() as u16 <= self.line_len {
                cur_line_len += self.qd[j].len() as u16 + 1;
                self.dline[self.dline.len() - 1].push_str(self.qd[j]);
                self.dline[self.dline.len() - 1].push(' ');
                self.tline[self.tline.len() - 1].push_str(self.qt[j]);
                self.tline[self.tline.len() - 1].push(' ');
            } else {
                self.line[self.line.len() - 1] = j as u16;
                break;
            }
        }
    }

    pub fn sync_on_resize(&mut self, frame_size: u16) {
        self.line_len = Que::set_line_len(frame_size);
        self.set_line();
    }

    #[inline]
    pub fn quit(&mut self) {
        self.quit = true;
    }

    #[inline]
    pub fn should_quit(&self) -> bool {
        self.quit
    }
}
