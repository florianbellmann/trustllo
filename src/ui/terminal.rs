
// /// Check and report to the user if the current environment is not a terminal.
// pub fn check_if_terminal() {
//     use crossterm::tty::IsTty;

//     if !stdout().is_tty() {
//         eprintln!(
//             "Warning: bottom is not being output to a terminal. Things might not work properly."
//         );
//         eprintln!("If you're stuck, press 'q' or 'Ctrl-c' to quit the program.");
//         stderr().flush().unwrap();
//         thread::sleep(Duration::from_secs(1));
//     }
// }
