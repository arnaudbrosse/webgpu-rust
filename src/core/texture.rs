pub struct Texture {
    width: u32,
    height: u32,
    data: Vec<u8>,
}

impl Texture {
    pub fn new(width: u32, height: u32, data: Vec<u8>) -> Self {
        Texture {
            width,
            height,
            data,
        }
    }
}
