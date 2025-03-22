#![allow(unused)]

use std::collections::HashMap;

#[derive(Debug)]
pub struct Body<T> {
    body: Option<T>,
}

impl<T: AsRef<[u8]>> Body<T> {
    pub fn new() -> Self {
        Self { body: None }
    }

    // 转化为 Vec[u8]
    pub fn serialize(&self) -> Vec<u8> {
        self.body
            .as_ref()
            .map_or_else(Vec::new, |b| [b.as_ref(), b"\r\n"].concat()) // 等价于match
    }

    pub fn set_body(&mut self, val: T) {
        self.body = Some(val);
    }

    pub(crate) fn len(&self) -> usize {
        self.serialize().len()
    }
}

#[derive(Debug)]
pub struct Json(HashMap<Tree, String>);

#[derive(Debug)]
pub struct Tree(HashMap<String, String>);

#[cfg(test)]
mod tests {
    use super::Body;

    #[test]
    fn test1() {
        let body = Body {
            body: Some("haha".to_string()),
        };
        println!("{:?}", body.serialize());
    }
}
