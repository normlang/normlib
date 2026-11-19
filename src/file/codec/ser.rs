pub trait Serialize {
    fn serialize(&self, buf: &mut Vec<u8>);
}

impl Serialize for u16 {
    fn serialize(&self, buf: &mut Vec<u8>) {
        buf.append(&mut self.to_le_bytes().to_vec());
    }
}

impl Serialize for str {
    fn serialize(&self, buf: &mut Vec<u8>) {
        let size = self.len() as u16;
        buf.append(&mut size.to_le_bytes().to_vec());
        buf.extend_from_slice(self.as_bytes());
    }
}

impl Serialize for String {
    fn serialize(&self, buf: &mut Vec<u8>) {
        self.as_str().serialize(buf)
    }
}
