pub mod wordlist; // List of english words and their phoneme sequences

use rodio::{Decoder, OutputStream, Sink, Source};
use rust_embed::RustEmbed;
use std::io::Cursor;
use std::time::Duration;

// Include the relevant tones of the Mark voice into the executable
#[derive(RustEmbed)]
#[folder = "ITS_TTS/mark/anger1"]
struct MarkAnger1;

#[derive(RustEmbed)]
#[folder = "ITS_TTS/mark/trust1"]
struct MarkTrust1;

#[derive(RustEmbed)]
#[folder = "ITS_TTS/mark/trust2"]
struct MarkTrust2;

fn main() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default()
        .unwrap();
    // Define an audio sink which uses the stream handle
    let sink = Sink::try_new(&stream_handle)
        .unwrap();

    
    // Test: Iterate over the entire phoneme set
    // This loop will eventually process each phoneme per word
    for (i, phoneme) in MarkTrust1::iter().enumerate() {
        println!("{}", phoneme);
                
        // Get a file-like Cursor to the audio data from the phoneme's path.
        let file = Cursor::new(MarkTrust1::get(&phoneme)
            .unwrap()
            .data);
        // Decode that sound file into a source
        let source = Decoder::new_vorbis(file)
            .unwrap();

        // Add the source to the sink queue. This starts playback asyncronously.
        match i {
            // Add a pause to the start of each word by matching the first phoneme
            0=>sink.append(source.delay(Duration::from_millis(65))),
            _=>sink.append(source) // Any other phoneme should not have a delay
        };
    }

    // Await completion of playback
    sink.sleep_until_end();
}
