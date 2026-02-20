
use core::ops::{Sub, Mul, AddAssign};


pub struct Yin<T, const N: usize> {

    pub result: [T; N],
}

#[derive(Debug)]
pub enum Error{
    /// Frame size should be 2 * result size
    FrameSizeMismatch,
}

pub trait YinCalculator<T> {

    fn calculate(&mut self, frame: &[T]) -> Result<(), Error>;
}

impl<T, const N: usize> YinCalculator<T> for Yin<T, N>
where T: From<u8> + Sub<Output = T> + Mul<Output = T> + AddAssign + Copy{
    
    fn calculate(&mut self, frame: &[T]) -> Result<(), Error>{
        
        if frame.len() != N * 2 {
            return Err(Error::FrameSizeMismatch);
        }
        
        for t in 0..frame.len()/2 {

            let mut sum = 0_u8.into();
            for j in 0..frame.len()/2 {

                let diff = frame[j] - frame[j+t];
                let square = diff * diff;
                sum += square;
            }
            self.result[t] = sum;
        }

        Ok(())
    }
}

impl<T, const N: usize> Yin<T, N>
where T: Copy + From<u8> {

    pub fn new() -> Self {
        Yin { result: [1_u8.into(); N] }
    }
}
