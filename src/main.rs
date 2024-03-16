use rodio::{source::Repeat, source::Source, Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;

fn load_sound(name: String) -> Repeat<Decoder<BufReader<File>>> {
    println!("loading file");
    let file = BufReader::new(File::open(name).unwrap());
    // Decode that sound file into a source
    println!("decoding file");
    Decoder::new(file).unwrap().repeat_infinite()
}

fn main() {
    let source = load_sound("rabarber.mp3".to_string());
    let (_stream, stream_handle) = OutputStream::try_default().unwrap();
    let sink = Sink::try_new(&stream_handle).unwrap();
    sink.append(source);
    sink.play();

    std::thread::park();
}