
use core::ops::{AddAssign, Div, DivAssign, Mul, Sub};
pub use fugit::{HertzU32 as Hertz, MicrosDurationU32 as MicroSecond, ExtU32, RateExtU32};
pub use fixed::types::I50F14;
pub use heapless::{self, Vec};
pub use pitchy::Pitch;


pub struct Yin<T, const N: usize> {

    diff_function: [T; N],
    threshold: T,
    sample_rate: f32,
}

#[derive(Debug, PartialEq)]
pub enum Error{
    /// Frame size should be 2 * diff_function size
    FrameSizeMismatch,
}

/// Default data type
pub type Sample = I50F14;
/// Output data type
pub type Candidates = heapless::Vec<Pitch, 5>;

pub trait YinCalculator<T, const N: usize> {

    /// Calculate difference function according to YIN method
    fn calculate_diff(&mut self, frame: &[T]) -> Result<(), Error>;

    /// Return a list of pitch candidates with diff dunction values below threshold
    fn find_candidates(&self) -> Candidates;
}

impl<T, const N: usize> YinCalculator<T, N> for Yin<T, N>
where T: From<u8> + 
Sub<Output = T> + 
Mul<Output = T> + 
AddAssign + 
Copy + 
Div<Output = T> + 
DivAssign + 
PartialOrd {
    
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

    fn find_candidates(&self) -> Candidates {
        find_candidates_inner::<T, N>(&self.diff_function, self.threshold, self.sample_rate)
    }
}

impl<T, const N: usize> Yin<T, N>
where T: Copy + From<u8> + DivAssign {

    pub fn new_with_sample_rate(sample_rate: Hertz) -> Self {
        let sample_rate = sample_rate.to_Hz() as f32;
        Self::new_inner(sample_rate)
    }

    pub fn new_with_sample_period(sample_period: MicroSecond) -> Self {
        let sample_rate = 1_000_000.0 / sample_period.to_micros() as f32;
        Self::new_inner(sample_rate)
    }

    pub fn set_threshold(&mut self, threshold: T) {
        self.threshold = threshold;
    }

    pub fn diff_function(&self) -> &[T; N]{
        &self.diff_function
    }

    fn new_inner(sample_rate: f32) -> Self {
        // Default threshold value = 0.15
        let mut threshold: T = 15_u8.into();
        threshold /= 100_u8.into();

        Yin { diff_function: [1_u8.into(); N], threshold, sample_rate }
    }
}

    
fn find_candidates_inner<T, const N: usize>(diff_function: &[T; N], threshold: T, sample_rate: f32) -> Candidates
where T: PartialOrd + Copy {
    let mut candidates: Candidates = Vec::new();

    #[derive(PartialEq)]
    enum State {
        Idle,
        Candidate,
    }

    let mut state = State::Idle;
    let mut min_value = threshold;
    let mut min_index = 1_usize;
    for i in 1..diff_function.len() {
        // Value under threshold: pitch candidate
        if state == State::Idle && diff_function[i] < threshold {
            state = State::Candidate;
        }
        if state == State::Candidate {
            // Find min value corresponding to pitch frequency
            if diff_function[i] < min_value {
                min_value = diff_function[i];
                min_index = i;
            }
            // Collect found min value
            if diff_function[i] > threshold {
                state = State::Idle;
                min_value = threshold;

                let frequency = sample_rate as f64 / min_index as f64;
                let candidate = Pitch::new(frequency);
                
                candidates.push(candidate).ok();
                if candidates.is_full() {
                    break;
                }
            }
        }
    }

    candidates
}


#[cfg(test)]
mod tests {
    use super::*;
    use pretty_assertions::{assert_eq, assert_ne};
    

    #[test]
    fn correct_frame_size() {

        const FRAME_SIZE: usize = 256;
        const RES_SIZE: usize = FRAME_SIZE / 2;
        let frame = [Sample::from_num(0); FRAME_SIZE];

        let mut yin:Yin<Sample, RES_SIZE> = Yin::new_with_sample_rate(1000.Hz());

        assert_eq!(yin.calculate_diff(&frame), Ok(()));
    }
}
