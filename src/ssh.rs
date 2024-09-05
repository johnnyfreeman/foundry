use ssh2::Session;
use std::io::Read;
use std::net::TcpStream;
use std::path::Path;

pub struct SshConnection {
    session: Session,
}

impl SshConnection {
    pub fn connect(
        host: &str,
        username: &str,
        key_path: &Path,
    ) -> Result<Self, Box<dyn std::error::Error>> {
        let tcp = TcpStream::connect(host)?;
        let mut session = Session::new()?;
        session.set_tcp_stream(tcp);
        session.handshake()?;
        session.userauth_pubkey_file(username, None, key_path, None)?;

        if !session.authenticated() {
            return Err("SSH authentication failed".into());
        }

        Ok(SshConnection { session })
    }

    pub fn execute_command(&self, command: &str) -> Result<String, Box<dyn std::error::Error>> {
        let mut channel = self.session.channel_session()?;
        channel.exec(command)?;
        let mut output = String::new();
        channel.read_to_string(&mut output)?;
        channel.wait_close()?;
        Ok(output)
    }
}
