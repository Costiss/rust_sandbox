struct CircularBuffer<T> {
    buffer: Vec<Option<T>>,
    size: usize,
    read_cursor: usize,
    write_cursor: usize,
}
enum CursorType {
    READ,
    WRITE,
}
impl<T: Clone> CircularBuffer<T> {
    pub fn new(size: usize) -> CircularBuffer<T> {
        CircularBuffer {
            buffer: vec![None; size],
            size,
            read_cursor: 0,
            write_cursor: 0,
        }
    }

    pub fn write(&mut self, data: T) -> Result<(), String> {
        match self.buffer[self.write_cursor] {
            Some(_) => Err("Full buffer".to_string()),
            None => {
                self.buffer[self.write_cursor] = Some(data);
                self.move_cursor(CursorType::WRITE);
                Ok(())
            }
        }
    }

    pub fn pop_older(&mut self) -> Result<T, String> {
        match &self.buffer[self.read_cursor] {
            None => Err("Empty Buffer".to_string()),
            Some(value) => {
                let tmp = value.clone();
                self.buffer[self.read_cursor] = None;
                self.move_cursor(CursorType::READ);
                Ok(tmp)
            }
        }
    }

    pub fn overwrite(&mut self, data: T) -> Result<T, String> {
        match &self.buffer[self.write_cursor] {
            None => Err("Empty Buffer".to_string()),
            Some(val) => {
                let tmp = val.clone();
                self.buffer[self.write_cursor] = Some(data);
                self.move_cursor(CursorType::WRITE);
                self.move_cursor(CursorType::READ);
                Ok(tmp)
            }
        }
    }

    fn move_cursor(&mut self, write_or_read: CursorType) {
        let cursor = match write_or_read {
            CursorType::READ => &mut self.read_cursor,
            CursorType::WRITE => &mut self.write_cursor,
        };
        *cursor += 1;
        if *cursor == self.size {
            *cursor = 0;
        }
    }
}

fn main() {
    let mut buffer: CircularBuffer<i32> = CircularBuffer::new(7);

    for i in 0..10 {
        if i == 8 {
            buffer.overwrite(i).unwrap();
            println!("OVEWRITTEN INT: {:?}", i);
            println!("FULL BUFFER: {:?}", buffer.buffer);
            continue;
        }
        if i == 9 {
            let pop = buffer.pop_older().unwrap();
            println!("POPPED INT: {:?}", pop);
            println!("FULL BUFFER: {:?}", buffer.buffer);
            break;
        }
        buffer.write(i).unwrap();
        println!("INSERTED INT: {:?}", i);
        println!("FULL BUFFER: {:?}", buffer.buffer);
        if i == 6 {
            let pop = buffer.pop_older().unwrap();
            println!("POPPED INT: {:?}", pop);
            println!("FULL BUFFER: {:?}", buffer.buffer);
        }
    }
}
