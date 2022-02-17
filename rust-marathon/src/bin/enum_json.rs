use serde::{Serialize, Deserialize};

const INPUT: &str = r#"
[
    {"type": "FTP", "host": "testurl.com", "file": "/path/to/file"},
    {"type": "HTTP", "host": "testurl3.com", "path":"/files/logs-02-02"},
    {"type": "SSH", "host": "testurl2.com", "login": "login", "password": "password", "file": "/path/to/file"},
    {"type": "SSH", "host": "testurl4.com", "auth": {"type": "Password", "login": "login", "password": "password"}, "file": "/path/to/file"},
    {"type": "SSH", "host": "testurl4.com", "auth": {"type": "Key", "path": "/path/to/key"}, "file": "/path/to/file"}
]
"#;

#[derive(Serialize, Deserialize, Debug)]
struct FtpItem {
    host: String,
    file: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct HttpItem {
    host: String,
    path: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SshPasswordItem {
    host: String,
    login: String,
    password: String,
    file: String,
}

#[derive(Serialize, Deserialize, Debug)]
struct SshAuthItem {
    host: String,
    auth: Auth,
    file: String,
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Item {
    FTP(FtpItem),
    HTTP(HttpItem),
    SSH(SshItem),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(untagged)]
enum SshItem {
    SshPassword(SshPasswordItem),
    SshAuth(SshAuthItem),
}

#[derive(Serialize, Deserialize, Debug)]
#[serde(tag = "type")]
enum Auth {
    Password,
    Key,
}

fn main() {
    let items: Vec<Item> = serde_json::from_str(&INPUT2).unwrap();

    for item in items {
        match item {
            Item::SSH(ssh) => {
                match ssh {
                    SshItem::SshPassword(sshPassword) => println!("password={:?}", sshPassword.password),
                    SshItem::SshAuth(sshAuth) => println!("auth={:?}", sshAuth.auth),
                }
            }
            _ => ()
        }
    }
}
