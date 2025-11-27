use lolcrab::Lolcrab;
use std::io;
use std::io::Write;
use std::process::Command;
use std::time::Duration;

const ANIMATION_TIME: u64 = 100;
pub fn ascii_art() {
    let mut lol = Lolcrab::new(None, None);
    let stdout = io::stdout();
    let mut stdout = stdout.lock();
    for c in 0..10 {
        clear_screen();
        lol.colorize_str(&asci(), &mut stdout);
        stdout.flush();
        sleep();
    }
    clear_screen();
    lol.colorize_str(&asci(), &mut stdout);
}
fn asci() -> &'static str {

    
    r#"
__________                __         ____  ___
\______   \_______ __ ___/  |_  ____ \   \/  /
 |    |  _/\_  __ \  |  \   __\/ __ \ \     / 
 |    |   \ |  | \/  |  /|  | \  ___/ /     \ 
 |______  / |__|  |____/ |__|  \___  >___/\  \
        \/                         \/      \_/
                                                     

 ➔ PhpMyAdmin
 ➔ WP-Login
 ➔ cPanel
 ➔ Webmail
 ➔ FTP
 ➔ MySQL
 ➔ Whm 
 ➔ Ssh 
                                           
Create -> Telegram : @federatic

    "#
}

fn clear_screen() {
    if cfg!(target_os = "windows") {
        Command::new("cmd")
            .args(&["/C", "cls"])
            .status()
            .expect("Error");
    } else {
        Command::new("clear").status().expect("Errro");
    }
}

fn sleep() {
    std::thread::sleep(Duration::from_millis(ANIMATION_TIME));
}


pub fn open_link() {
    let link = "https://t.me/@federatic";
    if let Err(err) = webbrowser::open(link) {
        eprintln!("Failed to open the link: {}", err);
    }
}
