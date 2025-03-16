use std::io::{Write, Read};
use std::net::TcpStream;
use super::user_agent::UserAgent;

pub struct SimpleUserAgent {
    pub stream: Option<TcpStream>,
    pub buffer: [u8; 512]
}

impl UserAgent for SimpleUserAgent {
    fn conn(&mut self) {
        let mut stream = TcpStream::connect("127.0.0.1:1025")
            .expect("Failed to connect to the SMTP server at 127.0.0.1:1025");

        let mut buffer = [0; 512];
        stream.read(&mut buffer)
            .expect("Failed to read response from the SMTP server");

        self.stream = Some(stream);
    }

    fn hello(&mut self){
        let stream = self.stream.as_mut().expect("Stream is not initialized. Call `conn` first.");

        stream.write_all(b"EHLO localhost\r\n")
        .expect("Failed to send EHLO command to the SMTP server");
    
        stream.read(&mut self.buffer)
        .expect("Failed to read response from the SMTP server");
    }

    fn from(&mut self, from: String) {
        let stream = self.stream.as_mut().expect("Stream is not initialized. Call `conn` first.");

        let command = format!("MAIL FROM:<{}>\r\n", from);
        stream.write_all(command.as_bytes())
            .expect("Failed to send FROM command to the SMTP server");
        stream.read(&mut self.buffer)
            .expect("Failed to read response from the SMTP server");
    }

    fn to(&mut self, to: String) {
        let stream = self.stream.as_mut().expect("Stream is not initialized. Call `conn` first.");

        let command = format!("RCPT TO:<{}>\r\n", to);
        stream.write_all(command.as_bytes())
            .expect("Failed to send FROM command to the RCPT server");
        stream.read(&mut self.buffer)
            .expect("Failed to read response from the SMTP server");
    }

    fn data(&mut self, data: String) {
        let stream = self.stream.as_mut().expect("Stream is not initialized. Call `conn` first.");

        stream.write_all(b"DATA\r\n")
            .expect("Failed to send DATA command to the SMTP server");
    
        stream.read(&mut self.buffer)
            .expect("Failed to read server response after DATA command");

        stream.write_all(data.as_bytes())
            .expect("Failed to send email body");
    
        stream.read(&mut self.buffer)
            .expect("Failed to read server response after sending email body");
    }
    
    fn quit(&mut self) {
        let stream = self.stream.as_mut().expect("Stream is not initialized. Call `conn` first.");

        stream.write_all(b"QUIT\r\n")
            .expect("Failed to send QUIT command to the SMTP server");
    
        stream.read(&mut self.buffer)
            .expect("Failed to read server response after QUIT command");

    }    
}
