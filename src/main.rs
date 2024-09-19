use kalosm::sound::*;

#[tokio::main]
async fn main() {
    // Transcription-model
    let model = Whisper::builder()
        .build_with_loading_handler(|progress| match progress {
            ModelLoadingProgress::Downloading {
                source,
                start_time,
                progress,
            } => {
                let progress = (progress * 100.0) as u32;
                let elapsed = start_time.elapsed().as_secs_f32();
                println!("Downloading file {source} {progress}% ({elapsed}s)");
            }
            ModelLoadingProgress::Loading { progress } => {
                let progress = (progress * 100.0) as u32;
                println!("Loading model {progress}%");
            }
        })
        .await
        .unwrap();

    // Stream audio from the microphone
    let mic = MicInput::default();
    let stream = mic.stream().unwrap();

    // Transcribe the audio.
    let mut transcribed = stream.transcribe(model);

    // As the model transcribes the audio, print the text to the console.
    transcribed.to_std_out().await.unwrap();
}
