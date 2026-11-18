pub trait Serialize {
    fn serialize(&self, buf: &mut Vec<u8>);
}

impl Serialize for u16 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.append(&mut self.to_le_bytes().to_vec());
    }
}
