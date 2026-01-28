use std::process::{Command, Stdio};
//use std::io::{BufReader, BufRead, BufWriter, Write, self, stdin};
//use std::fs::File;
//use std::thread;

// rtl_fm -f 94700000 -M wbfm | aplay -r 24000 -f S16_LE -t raw -c 1

// THIS ONE WORKED WITH HACKRF ONE
// rx_fm -f 94.7M -M wbfm -s 170k -r 32k - | play -r 32k -t raw -e signed-integer -b 16 -c 1 -

// added comment to track  in git
fn main() -> std::io::Result<()> {
    //let mut should_stop = false;

    // let mut receive_radio = Command::new("rx_fm -f 94.7M -M wbfm -s 170k -r 32k -")
    // 	.stdout(Stdio::piped())
    // 	.spawn()?;

    // let play_audio = Command::new("play -r 32k -t raw -e signed-integer -b 16 -c 1 -")
    // 	.stdin(Stdio::from(receive_radio.stdout.take().unwrap()))
    // 	.stdout(Stdio::piped())
    // 	.spawn()?;
    
    let mut receive_radio = Command::new("rx_fm")
        .arg("-f")
	.arg("94.7M")
	.arg("-M")
	.arg("wbfm")
	.arg("-s")
	.arg("170k")
	.arg("-r")
	.arg("32k")
	.arg("-")
        .stdout(Stdio::piped())
        .spawn()?;

    let play_audio = Command::new("play")
	.arg("-r")
	.arg("32k")
	.arg("-t")
	.arg("raw")
	.arg("-e")
	.arg("signed-integer")
	.arg("-b")
	.arg("16")
	.arg("-c")
	.arg("1")
	.arg("-")
	.stdin(receive_radio.stdout.take().unwrap())
	.stdout(Stdio::piped())
	.spawn()?;

    let _output = play_audio.wait_with_output()?;

    Ok(())
}

// fn main() -> std::io::Result<()> {
//     // 1. Spawn the first command (`echo`) and configure its stdout to be piped.
//     let mut hello = Command::new("echo")
//         .arg("Hello, world!")
//         .stdout(Stdio::piped()) // Capture stdout into a pipe
//         .spawn()?;

//     // 2. Configure the second command (`rev`) to use the first command's stdout as its stdin.
//     // The `hello.stdout` is an Option<ChildStdout>, so we unwrap it and convert to Stdio.
//     let reverse = Command::new("rev")
//         .stdin(hello.stdout.take().unwrap()) // Pipe the first command's stdout to this command's stdin
//         .stdout(Stdio::piped()) // Also pipe this command's stdout to capture later
//         .spawn()?;

//     // 3. Wait for the second command to complete and capture its output.
//     let output = reverse.wait_with_output()?;

//     // 4. Verify the output.
//     //assert_eq!(output.stdout, b"! dlrow ,olleH\n");
//     println!("Piped output: {}", String::from_utf8_lossy(&output.stdout));

//     Ok(())
// }
