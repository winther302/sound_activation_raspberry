use rodio::{source::Repeat, source::Source, Decoder, OutputStream, Sink};
use std::fs::File;
use std::io::BufReader;
use rppal::gpio::Gpio;
use std::{thread, time};

enum State {
    Open,
    Close,
}

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
    sink.pause();
    let gpio = Gpio::new().unwrap();
    let mut pin = gpio.get(23).unwrap().into_input_pullup();
    let state: State = State::Open;
    let debounce = time::Duration::from_millis(10);
    loop {
        match (pin.is_high(), sink.is_paused()) {
            (true, true) => {
                println!("starting file");
                sink.play();
            },
            (false, false) => {
                println!("pause file");
                sink.pause()
            },
            (_, _) => ()
        }
        thread::sleep(debounce);
    }
}
