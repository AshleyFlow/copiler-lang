pub struct Cursor<T> {
    items: Vec<T>,
    pos: usize,
}

impl<T> Cursor<T>
where
    T: PartialEq<T>,
{
    pub fn new(items: Vec<T>) -> Self {
        Self { items, pos: 0 }
    }

    pub fn eat(&mut self) {
        self.pos += 1;
    }

    pub fn eat_if(&mut self, expected: T) -> bool {
        let peeked = self.peek(None);

        if peeked.is_some() && *peeked.unwrap() == expected {
            self.pos += 1;

            true
        } else {
            false
        }
    }

    pub fn current(&self) -> &T {
        &self.items[self.pos]
    }

    pub fn peek(&self, offset: Option<usize>) -> Option<&T> {
        let offset = offset.unwrap_or(1);
        let pos = self.pos + offset;

        if pos < self.items.len() {
            Some(&self.items[pos])
        } else {
            None
        }
    }
}
