mod wordlist; // Contains list of English words and their phoneme tokens

use rust_embed::RustEmbed;
use std::io;
use std::time::Duration;
use rodio::{Decoder, OutputStream, Sink, Source};
use rand::seq::SliceRandom;

// Include the Mark voice into the executable
#[derive(RustEmbed)]
#[folder = "ITS_TTS/mark"]
struct MarkVoice;

fn main() {
    // Get a output stream handle to the default physical sound device
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    // Define an audio sink which uses the stream handle
    let sink = Sink::try_new(&stream_handle).unwrap();

    for line in io::stdin().lines() {
        let line_text = line.unwrap();
        println!("\r{}", &line_text);
        let input_upper = line_text.to_uppercase();
        let sentence = input_upper.split_whitespace();

        // Iterate over each word, get its phonemes, and run the loop on those phonemes
        for word in sentence {
            let tokens = wordlist::get_phonemes(word).unwrap();
            //println!("{}: {:?}", word, tokens);

            // This loop processes each phoneme per word
            for (i, phoneme) in tokens.iter().enumerate() {
                //println!("{:?}", phoneme);

                // Construct the path to the audio data
                let mut path: String = "trust1/".to_owned();
                path.push_str(format!("{:?}",phoneme).as_str());
                path.push_str("/");
                //path.push_str(wordlist::get_filename(&phoneme).unwrap());
                // Pick a file at random
                let names = wordlist::get_filenames(&phoneme);
                let name = names.choose(&mut rand::thread_rng());
                assert!(name!=None);
                path.push_str(name.unwrap());

                path.push_str(".ogg");
                //println!("{}",path);
            
                // Get a file-like Cursor to the audio data from the phoneme's path.
                let file = io::Cursor::new(MarkVoice::get(&path).unwrap().data);
                // Decode that sound file into a source
                let source = Decoder::new_vorbis(file).unwrap().speed(1.1);

                // Add the source to the sink queue. This starts playback asyncronously.
                match i {
                    // Add a pause to the start of each word by matching the first phoneme
                    0 => sink.append(source.delay(Duration::from_millis(35))),
                    _ => sink.append(source), // Any other phoneme should not have a delay
                };
            }

            // Await completion of word playback
            sink.sleep_until_end();
        }
    }
}
