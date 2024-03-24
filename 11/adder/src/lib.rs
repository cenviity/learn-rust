#[derive(Debug)]
struct Rectangle {
    width: u32,
    height: u32,
}

impl Rectangle {
    fn can_hold(&self, other: &Self) -> bool {
        self.width > other.width && self.height > other.height
    }
}

#[cfg(test)]
mod tests {}
