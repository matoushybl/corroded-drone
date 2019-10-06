use std::collections::HashMap;
use std::io::{Cursor, Seek, SeekFrom, Write};

// There are chunks of data, each chunk consists of 8 fragments, each fragment takes up-to 1024 bytes
struct DownloadedFile {
    filenum: u16,
    size: u32,
    received_bytes: u32,
    received_chunk_numbers: HashMap<u32, u8>,
    number_of_chunks: u32,
    buffer: Cursor<Vec<u8>>,
}

impl DownloadedFile {
    fn new(filenum: u16, size: u32) -> Self {
        DownloadedFile {
            filenum,
            size,
            received_bytes: 0,
            received_chunk_numbers: HashMap::new(),
            number_of_chunks: ((size / 1024 + 1) / 8 + 1),
            buffer: Cursor::new(Vec::new()),
        }
    }

    pub fn is_finished(&self) -> bool {
        return self.received_bytes >= self.size;
    }

    fn has_fragment(&mut self, chunk_number: u32, fragment_number: u8) -> bool {
        *(self.received_chunk_numbers.entry(chunk_number).or_insert(0))
            & (1 << (fragment_number % 8))
            > 0
    }

    pub fn receive_fragment(
        &mut self,
        chunk_number: u32,
        fragment_number: u8,
        size: u16,
        data: &[u8],
    ) -> bool {
        if self.has_fragment(chunk_number, fragment_number) {
            return false;
        }

        // Mark a fragment as received.
        // Returns true if we have all fragments making up that chunk now.
        self.buffer
            .seek(SeekFrom::Start(fragment_number as u64 * 1024));
        self.buffer.write(data);
        self.received_bytes += size as u32;
        *(self.received_chunk_numbers.entry(chunk_number).or_insert(0)) |=
            1 << (fragment_number % 8);

        return self.received_chunk_numbers[&chunk_number] == 0xFF;
    }

    pub fn get_data(&self) -> Vec<u8> {
        (*self.buffer.get_ref()).to_vec()
    }
}

mod tests {
    use super::DownloadedFile;

    #[test]
    fn download() {
        let mut file = DownloadedFile::new(0, 4 * 8 * 2);
        for chunk_number in 0u32..2u32 {
            for fragment_number in 0u8..8u8 {
                file.receive_fragment(chunk_number, fragment_number, 4, &[1, 2, 3, 4]);
            }
        }
        assert_eq!(file.received_bytes, file.size);
    }
}
