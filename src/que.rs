use crate::App;
use std::collections::VecDeque;

#[derive(Debug)]
pub struct Que<'a> {
    q: VecDeque<&'a (String, String)>,
    line: [u16; 3],
    line_len: u16,
    quit: bool,
}

impl<'a> Que<'a> {
    pub fn new(app: &'a App, frame_size: u16) -> Que<'a> {
        let mut nq = Que {
            q: VecDeque::with_capacity(128),
            line: [0; 3],
            line_len: Que::set_line_len(frame_size),
            quit: false,
        };
        for _ in nq.q.len()..nq.q.capacity() {
            nq.q.push_back(app.get_pair());
        }
        nq.set_line();
        nq
    }

    #[inline]
    fn set_line_len(frame_size: u16) -> u16 {
        frame_size
    }

    #[inline]
    fn set_line(&mut self) {
        let mut index = 0usize;
        for i in self.line.iter_mut() {
            let mut cur_line_len = 0u16;
            for j in index..self.q.capacity() {
                if cur_line_len + self.q[j].0.len() as u16 <= self.line_len {
                    cur_line_len += self.q[j].0.len() as u16 + 1;
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
            self.q.pop_front();
            self.q.push_back(app.get_pair());
        }
        // syncing line indexes
        for i in 0..self.line.len() - 1 {
            self.line[i] = self.line[i + 1] - self.line[i];
        }
        // syncing last line index
        let mut cur_line_len = 0;
        for j in self.line[self.line.len() - 2] as usize..self.q.capacity() {
            if cur_line_len + self.q[j].0.len() as u16 <= self.line_len {
                cur_line_len += self.q[j].0.len() as u16 + 1;
            } else {
                self.line[self.line.len() - 2] = j as u16;
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
