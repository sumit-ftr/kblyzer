use ratatui::prelude::Rect;
use ratatui::Frame;
use std::collections::HashMap;

pub struct State<'a> {
    pub pos: HashMap<u8, Rect>,
    pub pre_ch: Option<u8>,
    pub cur_ch: u8,
    pub space_count: u8,
    pub default_word: &'a String,
    pub target_word: &'a String,
    pub frame: &'a Frame<'a>,
}

impl<'a> State<'a> {}
