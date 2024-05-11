pub struct Cursor<T> {
    items: Vec<T>,
    pos: usize,
}

impl<T> Cursor<T>
where
    T: PartialEq<T> + Clone,
{
    pub fn new(items: Vec<T>) -> Self {
        Self { items, pos: 0 }
    }

    pub fn eat(&mut self) -> Option<T> {
        let peeked = self.peek(None);

        if peeked.is_some() {
            self.pos += 1;

            Some(self.items[self.pos].clone())
        } else {
            None
        }
    }

    pub fn eat_if(&mut self, expected: T) -> Option<T> {
        let peeked = self.peek(None);

        if peeked.is_some() && peeked.unwrap() == expected {
            self.pos += 1;

            Some(self.items[self.pos].clone())
        } else {
            None
        }
    }

    pub fn peek(&self, offset: Option<usize>) -> Option<T> {
        let offset = offset.unwrap_or(1);
        let pos = self.pos + offset;

        if pos < self.items.len() {
            Some(self.items[pos].clone())
        } else {
            None
        }
    }
}
