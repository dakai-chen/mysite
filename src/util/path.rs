/// 路径分隔符
const SEP: char = '/';

pub fn is_safe(path: &str) -> bool {
    for seg in path.split(SEP) {
        if seg.starts_with("..") {
            return false;
        } else if seg.contains('\\') {
            return false;
        } else if cfg!(windows) && seg.contains(':') {
            return false;
        }
    }
    true
}

pub fn root(root: impl Into<String>) -> Root {
    Root::from(root)
}

#[derive(Debug, Clone)]
pub struct Root(String);

impl Root {
    pub fn new() -> Self {
        Self(String::new())
    }

    pub fn from(root: impl Into<String>) -> Self {
        Self(root.into())
    }

    pub fn join<T: AsRef<str>>(mut self, src: T) -> Self {
        let src = src.as_ref();
        let src = if src.starts_with(SEP) { &src[1..] } else { src };

        if !self.0.is_empty() && !self.0.ends_with(SEP) {
            self.0.push(SEP);
        }

        self.0.push_str(src);
        self
    }

    pub fn into_string(self) -> String {
        self.0
    }
}

impl std::fmt::Display for Root {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(&self.0)
    }
}

pub fn extension(name: &str) -> &str {
    name.rsplit_once(".").map_or_else(|| "", |(_, v)| v)
}
