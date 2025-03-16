use std::io::{self, Write, Read};
use std::net::TcpStream;

fn send_email() -> io::Result<()> {
    // Conectar ao servidor SMTP (MailHog) na porta 1025
    let mut stream = TcpStream::connect("127.0.0.1:1025")?;
    println!("Conectado ao servidor SMTP!");

    // Ler a resposta inicial do servidor
    let mut buffer = [0; 512];
    stream.read(&mut buffer)?;
    println!("Resposta do servidor: {}", String::from_utf8_lossy(&buffer));

    // Enviar comando EHLO (identificação do cliente)
    stream.write_all(b"EHLO localhost\r\n")?;
    stream.read(&mut buffer)?;
    println!("Resposta do servidor: {}", String::from_utf8_lossy(&buffer));

    // Enviar comando MAIL FROM (remetente)
    stream.write_all(b"MAIL FROM:<remetente@example.com>\r\n")?;
    stream.read(&mut buffer)?;
    println!("Resposta do servidor: {}", String::from_utf8_lossy(&buffer));

    // Enviar comando RCPT TO (destinatário)
    stream.write_all(b"RCPT TO:<destinatario@example.com>\r\n")?;
    stream.read(&mut buffer)?;
    println!("Resposta do servidor: {}", String::from_utf8_lossy(&buffer));

    // Enviar comando DATA (iniciar o corpo do e-mail)
    stream.write_all(b"DATA\r\n")?;
    stream.read(&mut buffer)?;
    println!("Resposta do servidor: {}", String::from_utf8_lossy(&buffer));

    // Enviar o corpo do e-mail
    let email_body = "From: remetente@example.com\r\n\
                     To: destinatario@example.com\r\n\
                     Subject: Teste de E-mail\r\n\
                     \r\n\
                     Este é um e-mail de teste enviado via SMTP raw com Rust.\r\n\
                     .\r\n"; // O ponto final indica o fim do corpo do e-mail
    stream.write_all(email_body.as_bytes())?;
    stream.read(&mut buffer)?;
    println!("Resposta do servidor: {}", String::from_utf8_lossy(&buffer));

    // Enviar comando QUIT (encerrar a conexão)
    stream.write_all(b"QUIT\r\n")?;
    stream.read(&mut buffer)?;
    println!("Resposta do servidor: {}", String::from_utf8_lossy(&buffer));

    Ok(())
}