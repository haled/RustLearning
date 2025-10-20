use std::process::{Command, Stdio};
use std::io::{BufReader, BufRead, BufWriter, Write, self, stdin};
use std::fs::File;
use std::thread;

fn main() -> std::io::Result<()> {
    //let mut should_stop = false;
    
    let mut outside_ping = Command::new("ping")
        .arg("google.com")
        .stdout(Stdio::piped())
        //.stdin(Stdio::piped())
        .spawn()?;

    let output = outside_ping.stdout.take().expect("Failed to open stdout");
    //let input = outside_ping.stdin.take().expect("Failed to open stdin");

    // let input_thread = thread::spawn(move || {
    //     let mut input_line = String::new();
    //     loop {
    //         io::stdin().read_line(&mut input_line).expect("Failed to capture keystrokes");
    //         println!("\t**** {}", input_line);
    //         // figure out how to break loop
    //         if input_line ==  "q" {should_stop = true};
    //         input_line.clear();
    //     }
    //);
    
    let reader = BufReader::new(output);
    let dest_file = File::create("outside_output.txt")?;
    let mut writer = BufWriter::new(dest_file);
    
    for line in reader.lines() {
        let line = line?;
        println!("{}", line);
        let mod_line = "\nDATE/TIME GOES HERE ".to_string() + &line;
        writer.write_all(mod_line.as_bytes())?;
        //if should_stop { break };
        let _ = writer.flush();
    }

    //input_thread.join().expect("Thread failed.");

    Ok(())
}
