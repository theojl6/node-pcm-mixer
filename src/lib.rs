use neon::prelude::*;
use neon::types::buffer::TypedArray;

const SAMPLES_PER_FRAME: usize = 4;

// Use #[neon::export] to export Rust functions as JavaScript functions.
// See more at: https://docs.rs/neon/latest/neon/attr.export.html

// #[neon::export]
// fn hello(name: String) -> String {
//     format!("hello {name}")
// }

// Use #[neon::main] to add additional behavior at module loading time.
// See more at: https://docs.rs/neon/latest/neon/attr.main.html

#[neon::main]
fn main(mut cx: ModuleContext) -> NeonResult<()> {
    cx.export_function("mixLpcm", mix_lpcm)?;
    Ok(())
}

fn mix_lpcm(mut cx: FunctionContext) -> JsResult<JsBuffer> {
    let js_buffer_1 = cx.argument::<JsBuffer>(0)?;
    // Convert the JsBuffer to a Rust byte slice (&[u8])
    let byte_slice_1: &[u8] = js_buffer_1.as_slice(&cx);
    // Convert the byte slice to a Vec<u8>
    let bytes_1: Vec<u8> = byte_slice_1.to_vec();
    let js_buffer_2 = cx.argument::<JsBuffer>(1)?;
      // Convert the JsBuffer to a Rust byte slice (&[u8])
    let byte_slice_2: &[u8] = js_buffer_2.as_slice(&cx);
    // Convert the byte slice to a Vec<u8>
    let bytes_2: Vec<u8> = byte_slice_2.to_vec();

    let mixed_bytes = mix_bytes(bytes_1, bytes_2);

    let mut js_buffer = cx.buffer(mixed_bytes.len())?;

    // 3. Copy the data from the Rust Vec<u8> to the JsBuffer.
    // This is the step where the data is copied.
    for (i, elem) in js_buffer.as_mut_slice(&mut cx).iter_mut().enumerate() {
        *elem = mixed_bytes[i];
    }

    Ok(js_buffer)
}

fn mix_bytes(bytes_1: Vec<u8>, bytes_2: Vec<u8>) -> Vec<u8> {
    let mut a1_data: Vec<i16> = unsafe { bytes_1.align_to::<i16>() }.1.to_vec();
    let mut a2_data: Vec<i16> = unsafe { bytes_2.align_to::<i16>() }.1.to_vec();

    let a1_num_bytes = a1_data.len() * size_of::<i16>();
    let a2_num_bytes = a2_data.len() * size_of::<i16>();
    let num_bytes = if a1_num_bytes < a2_num_bytes {
        a1_data.resize(a2_data.len(), 0);
        a2_num_bytes
    } else {
        a2_data.resize(a1_data.len(), 0);
        a1_num_bytes
    };
    let num_samples = num_bytes / size_of::<i16>();
    let num_frames = num_samples / SAMPLES_PER_FRAME;
    let mut mixed_data = vec![0; num_samples];

    // pass each frame to the mixer
    for i in 0..num_frames {
        let start = i * SAMPLES_PER_FRAME;
        let end = i * SAMPLES_PER_FRAME + SAMPLES_PER_FRAME;
        mix_one_frame(SAMPLES_PER_FRAME, &mut mixed_data[start..end], 1, &a1_data[start..end], &a2_data[start..end]);
    }

    convert_i16_to_u8_bytes_le(mixed_data)
}

fn mix_one_sample(sample_1: i16, sample_2: i16) -> i16 {
    // normalization to save data range
    let sample_f1: f32 = (sample_1 as f32) / 32768.0f32;
    let sample_f2: f32 = (sample_2 as f32) / 32768.0f32;
    let mut mixed = sample_f1 + sample_f2;

    // reduce the volume a bit
    mixed = mixed * 0.9;

    // soft clipping
    if mixed > 1.0 {
        mixed = 1.0;
    }
    if mixed < -1.0 {
        mixed = -1.0;
    }

    let output_sample = (mixed * 32768.0f32) as i16;
    output_sample
}

fn mix_one_frame(frame_size: usize, out_frame: &mut [i16], num_channels: usize, frame_1: &[i16], frame_2: &[i16]) {
    let samples_per_frame = frame_size * num_channels;

    for i in 0..samples_per_frame {
        out_frame[i] = mix_one_sample(frame_1[i], frame_2[i]);
    }
    
}

fn convert_i16_to_u8_bytes_le(input_vec: Vec<i16>) -> Vec<u8> {
    let mut result_vec = Vec::new();
    for &val in input_vec.iter() {
        result_vec.extend_from_slice(&val.to_le_bytes()); // Little-endian
    }
    result_vec
}
