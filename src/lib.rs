use std::collections::HashMap;

#[derive(Debug)]
pub struct Mapping {
    mapping: HashMap<u8, u8>,
}

impl std::ops::Deref for Mapping {
    type Target = HashMap<u8, u8>;
    fn deref(&self) -> &Self::Target {
        &self.mapping
    }
}

impl std::ops::DerefMut for Mapping {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.mapping
    }
}

impl Mapping {
    pub fn new(mut a: std::env::Args) -> Result<Mapping, &'static str> {
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

        Ok(Mapping { mapping: hmap })
    }
}
