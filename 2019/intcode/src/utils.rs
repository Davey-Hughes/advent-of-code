use std::fmt::Write;

pub trait StringExt {
    fn expand_tabs(&self, tab_size: u8) -> String;
}

impl<T> StringExt for T
where
    T: AsRef<str>,
{
    fn expand_tabs(&self, tab_size: u8) -> String {
        let mut lines = vec![];
        for s in self.as_ref().split('\n') {
            let mut res = String::new();
            let mut last_pos = 0;

            while let Some(pos) = &s[last_pos..].find('\t') {
                res.push_str(&s[last_pos..*pos + last_pos]);

                let spaces_to_add = if tab_size != 0 {
                    tab_size as usize - (*pos % tab_size as usize)
                } else {
                    0
                };

                let _ = write!(res, "{:width$}", "", width = spaces_to_add);

                last_pos += *pos + 1;
            }

            res.push_str(&s[last_pos..]);

            lines.push(res);
        }

        lines.join("\n")
    }
}
