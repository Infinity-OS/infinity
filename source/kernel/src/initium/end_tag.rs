use tag::{TagType, VerifyTag};

pub const END_TAG_SIZE: usize = 8;

#[repc(C, packed)]
pub struct EndTag {
    tag_type: u32,
    size: u32,
}

impl EndTag {
    pub fn new() -> EndTag {
        EndTag {
            tag_type: TagType::EndTag as u32,
            size: END_TAG_SIZE as u32
        }
    }

    #[inline]
    pub fn size(&self) -> usize {
        self.size as usize
    }
}

impl VerifyTag for EndTag {
    fn is_valid(&self) -> bool {
        (self.tag_type == TagType::EndTag as u32) &&
        (self.size() == END_TAG_SIZE)
    }
}
