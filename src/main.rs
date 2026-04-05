use anyhow::{Context, Result, bail};
use clap::Parser;
use local_ip_address::local_ip;
use pulldown_cmark::{Options, html::push_html};
use qrcode::{QrCode, render::unicode};
use std::{fs, io::Write, net::Ipv4Addr, path::PathBuf};

/// `Brief-sv` is a minimal server for a single file.
/// In most environment such as Linux, Mac, Windows,
/// just run "brief-sv file.md" and you'll find an adorable qrcode below linked to the served file.
/// Note for WSL/WSL2 users: extra setup is required.
/// - setup firewall as explained in README.md
/// - run with --host option. You can ensure the value
///   by running "ipconfig" in your windows-terminal
///   and looking for ipaddress starting with "192.168..."
#[derive(Parser, Debug)]
#[command(version, about, long_about, verbatim_doc_comment)]
struct Args {
    /// Path to the file to serve
    /// Any text-base file is acceptable;
    /// .md will be automatically turned into .html
    file_path: PathBuf,
    /// Needless to use this option unless you're using WSL/WSL2
    #[arg(long)]
    host: Option<Ipv4Addr>,
    /// Usually not needed. If you change this, make sure the port is open in your firewall.
    #[arg(short, long)]
    port: Option<u16>,
}

fn main() -> Result<()> {
    let Args {
        file_path,
        host,
        port,
    } = Args::parse();

    let port = port.unwrap_or(7878);
    let is_md = Some("md")
        == file_path
            .extension()
            .context("target file requires extension")?
            .to_str();
    // fetch file
    let file_str = fs::read_to_string(&file_path)
        .with_context(|| format!("{} doesn't exist.", file_path.display()))?;

    let html = if is_md {
        let parser = pulldown_cmark::Parser::new_ext(&file_str, Options::all());
        let mut html_stack = String::new();
        html_stack
            .push_str("<meta charset=\"utf-8\"><meta name=\"viewport\" content=\"width=device-width, initial-scale=1.0\">");
        push_html(&mut html_stack, parser);
        html_stack
    } else {
        file_str
    };

    // show qr code to connect served file
    let local_ip_str = match host {
        Some(h) => h.to_string(),
        None => {
            if is_wsl() {
                bail!(
                    "You're using WSL/WSL2, but no --host option is given.\n Look at README.md for WSL/WSL2 setup section."
                );
            }
            local_ip()?.to_string()
        }
    };
    let url_str = format!("http://{}:{}", local_ip_str, port);
    let qrcode_data = QrCode::new(url_str.as_bytes())?;
    let qrcode_image = qrcode_data
        .render::<unicode::Dense1x2>()
        .dark_color(unicode::Dense1x2::Light)
        .light_color(unicode::Dense1x2::Dark)
        .build();
    println!("{}", qrcode_image);
    println!("Serving at: {}", url_str);

    // serve minimal tcp server with file content
    let listener = std::net::TcpListener::bind(format!("0.0.0.0:{}", port))?;
    for stream in listener.incoming() {
        let res_str = format!(
            "HTTP/1.1 200 OK\r\nContent-Type: text/html; charset=utf-8\r\nContent-Length: {len}\r\n\r\n{body}",
            len = html.len(), // String.len() returns byte length of the content
            body = html
        );
        stream?.write_all(res_str.as_bytes())?;
    }

    Ok(())
}

fn is_wsl() -> bool {
    let version_fetch_res = fs::read_to_string("/proc/version");
    match version_fetch_res {
        // linux(false) / wsl(true)
        Ok(v) => v.to_lowercase().contains("microsoft"),
        // windows(false) / mac(false)
        Err(_) => false,
    }
}
