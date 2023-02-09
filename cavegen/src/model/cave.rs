pub struct Cave {
    pub width: usize,
    pub height: usize,
    pub data: Vec<bool>,
}

impl Cave {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![false; width * height],
        }
    }
}
