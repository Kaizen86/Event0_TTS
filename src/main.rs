use rust_embed::RustEmbed;
use std::io::Cursor;
use rodio::{Decoder, OutputStream, Sink};

// Include the Mark voice into the executable
#[derive(RustEmbed)]
#[folder = "ITS_TTS/mark/"]
struct Voice;


fn main() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Define an audio sink which uses the stream handle
    let sink = Sink::try_new(&stream_handle).unwrap();

    // Test: Iterate over the entire phoneme set
    for phoneme in Voice::iter() {
        println!("{}",phoneme);
    
        // Get a file-like Cursor to the audio data from the phoneme's path.
        let file = Cursor::new(Voice::get(&phoneme).unwrap().data);// phoneme.as_bytes());
        // Decode that sound file into a source
        let source = Decoder::new_vorbis(file).unwrap();
        
        // Add the source to the sink queue. This starts playback asyncronously.
        sink.append(source);
    }

    // Await completion of playback
    sink.sleep_until_end();
}