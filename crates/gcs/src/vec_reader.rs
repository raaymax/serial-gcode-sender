use std::io::Read;
use std::io::Result;

pub struct VecReader(pub Vec<u8>);

impl Read for VecReader {
    fn read(&mut self, buf: &mut [u8]) -> Result<usize> {
        let len = buf.len();
        let available = self.0.len();
        let size = len.min(available);
        buf[..size].copy_from_slice(&self.0.as_slice()[..size]);
        let vec2 = self.0.split_off(size);
        self.0 = vec2;
        Ok(size)
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::io::BufReader;
    use std::io::BufRead;

    #[test]
    fn should_work() {
        let v = vec![1,2,3,4,5,6,7];
        let mut r = VecReader(v);
        let mut buf = [0u8; 5];
        let size = r.read(&mut buf).unwrap();
        assert_eq!(size, 5);
        assert_eq!(Vec::from(buf), vec![1,2,3,4,5]);
        let size2 = r.read(&mut buf).unwrap();
        assert_eq!(size2, 2);
        assert_eq!(Vec::from(&buf[..size2]), vec![6,7]);
    }

    #[test]
    fn should_work2() {
        let data = Vec::from("ajskldjaslkd\n asdasdaw\n asd sa\n");
        let mut r = BufReader::new(VecReader(data)).lines();
        let mut count = 0;
        while let Some(Ok(line)) = r.next() {
            assert!(line.len() > 4);
            count += 1;
        }
        assert_eq!(count, 3);
    }
}


