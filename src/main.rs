use std::boxed::Box;
use std::cmp;
use std::iter::{ Iterator, DoubleEndedIterator};

type Node = Option<u64>;

#[derive(Debug)]
pub struct TimestampSaver {
    buf: Box<[Node]>,
    cap: usize,
    pub length: usize,
}

pub struct ListIterator {
    current: usize,
    data: Box<[Node]>,
}

impl Iterator for ListIterator {
    type Item = u64;

    fn next(&mut self) -> Option<u64> {
        if self.current < self.data.len() {
            let item = self.data[self.current];
            self.current += 1;
            item
        }
        else{
            None
        }
    }
}

impl DoubleEndedIterator for ListIterator {
    fn next_back(&mut self) -> Option<u64> {
        if self.current < self.data.len() {
            let item = self.data[self.current];
            if self.current == 0 {
                self.current = self.data.len() - 1;
            }
            else{
                self.current -= 1;
            }
            item            
        }
        else{
            None
        }
    }
}

impl TimestampSaver {
    fn new(values: Vec<Option<u64>>) -> TimestampSaver {
        TimestampSaver {
            buf: values.into_boxed_slice(),
            cap: 0,
            length: 0
        }
    }

    fn grow(&mut self, min_cap: usize) {
        let old_cap = self.buf.len();
        let mut new_cap = old_cap + (old_cap >> 1);

        new_cap = cmp::max(new_cap,min_cap);
        new_cap = cmp::min(new_cap, usize::max_value());
        let current = self.buf.clone();
        self.cap = new_cap;

        self.buf = vec![None; new_cap].into_boxed_slice();
        self.buf[..current.len()].clone_from_slice(&current);
        self.length = self.buf.len()
    }

    pub fn at(&mut self, index:usize) -> Option<u64> {
        if self.length > index {
            self.buf[index]
        }
        else{
            None
        }
    }
}

fn main() {
    let mut timestamp_saver = TimestampSaver::new(
        vec![Some(1),Some(2),Some(3)]
    );
    timestamp_saver.grow(10);
    println!("Grow: {:#?}",timestamp_saver);
    println!("At: {:#?}",timestamp_saver.at(2));
}