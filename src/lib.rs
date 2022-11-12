pub struct MdFile<'a> {
    pub name: &'a str,
    pub content: String,
}

impl MdFile<'_> {
    pub fn add<T: std::fmt::Display>(&mut self, pre: &str, post: &str, dig: usize, val: T) -> T {
        self.content
            .push_str(&format!("{pre} {val:.dig$} {post}\n\n"));
        val
    }

    pub fn push_str(&mut self, s: &str) {
        self.content.push_str(s);
    }
}

impl Drop for MdFile<'_> {
    fn drop(&mut self) {
        std::fs::write(self.name, &self.content).unwrap();
    }
}

#[cfg(test)]
mod tests {
    // use super::*;

    #[test]
    fn it_works() {
        let result = 2 + 2;
        assert_eq!(result, 4);
    }
}
