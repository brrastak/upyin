
use core::ops::{AddAssign, Div, DivAssign, Mul, Sub};


pub struct Yin<T, const N: usize> {

    diff_function: [T; N],
}

#[derive(Debug, PartialEq)]
pub enum Error{
    /// Frame size should be 2 * diff_function size
    FrameSizeMismatch,
}

pub trait YinCalculator<T> {

    /// Calculate difference function according to YIN method
    fn calculate_diff(&mut self, frame: &[T]) -> Result<(), Error>;
}

impl<T, const N: usize> YinCalculator<T> for Yin<T, N>
where T: From<u8> + Sub<Output = T> + Mul<Output = T> + AddAssign + Copy + Div<Output = T> + DivAssign{
    
    fn calculate_diff(&mut self, frame: &[T]) -> Result<(), Error>{
        
        if frame.len() != N * 2 {
            return Err(Error::FrameSizeMismatch);
        }
        
        // Average squared difference over the window of N samples
        // diff_function(tau)
        for tau in 0..N {

            let mut sum = 0_u8.into();
            for j in 0..N {

                let diff = frame[j] - frame[j+tau];
                let square = diff * diff;
                sum += square;
            }
            self.diff_function[tau] = sum;
        }
        // Cumulative mean normalization
        let mut sum: T = 0_u8.into();
        let mut num: T = 0_u8.into();
        self.diff_function[0] = 1_u8.into();
        for value in self.diff_function.iter_mut() {
            sum += *value;
            num += 1_u8.into();
            let mean = sum / num;
            *value /= mean;
        }

        Ok(())
    }
}

impl<T, const N: usize> Yin<T, N>
where T: Copy + From<u8> {

    pub fn new() -> Self {
        Yin { diff_function: [1_u8.into(); N] }
    }

    pub fn diff_function(&self) -> &[T; N]{
        &self.diff_function
    }
}


#[cfg(test)]
mod tests {
    use super::*;
    use crate::Sample;
    use pretty_assertions::{assert_eq, assert_ne};
    

    #[test]
    fn correct_frame_size() {

        const FRAME_SIZE: usize = 256;
        const RES_SIZE: usize = FRAME_SIZE / 2;
        let frame = [Sample::from_num(0); FRAME_SIZE];

        let mut yin:Yin<Sample, RES_SIZE> = Yin::new();

        assert_eq!(yin.calculate_diff(&frame), Ok(()));
    }
}
