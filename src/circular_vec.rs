pub struct CircularVec {
    vec: Vec<(i16, i16, i16)>,
    current: usize,
}

impl CircularVec {
    pub fn new(vec: Vec<(i16, i16, i16)>) -> CircularVec {
        CircularVec { vec, current: 0 }
    }
    pub fn next(&mut self) -> (i16, i16, i16) {
        let prev = self.current;
        self.current += 1;
        if self.current >= self.vec.len() {
            self.current = 0
        }
        return self.vec[prev];
    }
}
