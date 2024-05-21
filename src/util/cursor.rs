pub trait ItemKind {
    fn kind(&self) -> u8;
}

pub struct Cursor<T> {
    items: Vec<T>,
    pos: usize,
}

impl<T> Cursor<T>
where
    T: Clone,
{
    pub fn new(items: Vec<T>) -> Self {
        Self { items, pos: 0 }
    }

    pub fn eat(&mut self) -> Option<T> {
        let peeked = self.peek(None);

        if peeked.is_some() {
            self.pos += 1;

            Some(self.items[self.pos - 1].clone())
        } else {
            None
        }
    }

    pub fn eat_iff(&mut self, handle: fn(T) -> bool) -> Option<T> {
        let peeked = self.peek(None);

        if peeked.is_some() && handle(peeked.unwrap()) {
            self.pos += 1;

            Some(self.items[self.pos - 1].clone())
        } else {
            None
        }
    }

    pub fn eat_if(&mut self, expected: T) -> Option<T>
    where
        T: ItemKind,
    {
        let peeked = self.peek(None);

        if peeked.is_some() && peeked.unwrap().kind() == expected.kind() {
            self.pos += 1;

            Some(self.items[self.pos - 1].clone())
        } else {
            None
        }
    }

    pub fn peek_iff(&mut self, offset: Option<usize>, handle: fn(T) -> bool) -> Option<T> {
        let offset = offset.unwrap_or(1);
        let pos = self.pos + offset;

        if pos <= self.items.len() {
            let peeked = &self.items[pos - 1];

            if handle(peeked.clone()) {
                Some(peeked.clone())
            } else {
                None
            }
        } else {
            None
        }
    }

    pub fn peek(&self, offset: Option<usize>) -> Option<T> {
        let offset = offset.unwrap_or(1);
        let pos = self.pos + offset;

        if pos <= self.items.len() {
            Some(self.items[pos - 1].clone())
        } else {
            None
        }
    }
}
