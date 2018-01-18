extern crate chrono;
extern crate irc;

use chrono::Utc;
use irc::client::prelude::*;
use irc::error::Error as IrcError;

use std::fs::OpenOptions;
use std::io::Write;

fn log_irc_message(message: Message) -> Result<(), IrcError> {
    let dt = Utc::now();
    let date_string = dt.format("%Y-%m-%d").to_string();
    // TODO: store logs in 'logs/' folder relative to current directory
    let mut file =
        OpenOptions::new()
        .create(true)
        .write(true)
        .append(true)
        .open(format!("irc-logger_{}.log", date_string))?;

    writeln!(file, "--- @ {}", dt)?;
    writeln!(file, "  pre: {:?}", message.prefix)?;
    writeln!(file, "  cmd: {:?}", message.command)?;
    writeln!(file, "")?;
    Ok(())
}

fn main() {
    let server = IrcServer::new("irc-logger.config.toml").unwrap();
    server.identify().unwrap();
    let messages_to_log = server.stream().filter(|message| {
        match message.command {
            Command::Response(Response::RPL_MOTD, _, _) 
            | Command::Response(Response::RPL_NAMREPLY, _, _) => false,
            _ => true,
        }
    });
    messages_to_log.for_each(|message| {
        log_irc_message(message)
    }).wait().unwrap();
}
