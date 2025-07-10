pub trait Prepare {
    fn prepare(&mut self, sample_rate: usize);
}

pub trait Process<I, O> {
    fn process(&mut self, input: &I) -> O;
    fn batch(&mut self, input: &[I], output: &mut [O]) {
        for (idx, val) in input.iter().enumerate() {
            output[idx] = self.process(val);
        }
    }
}
