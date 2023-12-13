pub struct MillIterator {
    current_alignment: u8,
    current_ring: u8,
    current_side: u8
}

impl MillIterator {
    fn new() -> Self {
        Self {
            current_alignment: 0,
            current_ring: 0,
            current_side: 0
        }
    }
}

impl Iterator for MillIterator {
    type Item = (u8, u8, u8);

    fn next(&mut self) -> Option<Self::Item> {
        let enumerate_alignments = self.current_alignment <= 3;
        if enumerate_alignments {
            // Enumerate mills along the leading lines of the board
            let alignment_offset = self.current_alignment * 2;
            let result = (
                alignment_offset,
                alignment_offset + 8,
                alignment_offset + 16
            );
            self.current_alignment += 1;
            result
        } else {
            // Enumerate mills along the rings
            if self.current_side <= 3 {
                // Enumerate mills along this ring
                self.current_side += 1;
            } else if self.current_ring <= 2 {
                // Go to a new ring
                self.current_ring += 1;
                self.current_side = 0;
            } else {
                None
            }

            todo!()
        }

        todo!()
    }
}