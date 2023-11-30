use rustfft::{num_complex::Complex, FftPlanner};

const WINDOW_SIZE: usize = 10;
const HZ: usize = 1;

fn main() {
    println!("Calculate Amplitude: An attempt to get the highest amplitude of a sine wave from a Fourier transform, regardless of phase");

    let source_tone_component = Complex::from_polar(1.0, 0.3);

    let mut source_signal = vec![Complex {im: 0.0, re: 0.0}; 10];
    source_signal[HZ] = source_tone_component;
    source_signal[WINDOW_SIZE - HZ] = Complex {
        re: source_tone_component.re,
        im: -1.0 * source_tone_component.im
    };

    let mut planner = FftPlanner::new();
    let fft_forward = planner.plan_fft_forward(WINDOW_SIZE);
    let fft_inverse = planner.plan_fft_inverse(WINDOW_SIZE);

    let mut scratch_forward = vec![
        Complex {
            re: 0.0f32,
            im: 0.0f32
        };
        fft_inverse.get_inplace_scratch_len()
    ];

    let mut scratch_inverse = vec![
        Complex {
            re: 0.0f32,
            im: 0.0f32
        };
        fft_inverse.get_inplace_scratch_len()
    ];

    fft_inverse.process_with_scratch(&mut source_signal, &mut scratch_inverse);

    for iteration in 0..WINDOW_SIZE {
        let mut transformed_signal = source_signal.clone();
        fft_forward.process_with_scratch(&mut transformed_signal, &mut scratch_forward);

        let (amplitude, phase) = transformed_signal[HZ].to_polar();

        println!("Iteration: {}; Amplitude: {}; Phase: {}", iteration, amplitude, phase);

        let sample_to_move = source_signal.remove(0);
        source_signal.push(sample_to_move);
    }
}
