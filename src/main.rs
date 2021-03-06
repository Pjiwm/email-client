extern crate imap;
extern crate native_tls;
mod app;
mod handlers;
mod layout;
use handlers::{
    email_handler::{Email, SmtpConnectionManager},
    file_handler::FileManager,
    imap_handler,
};
fn main() {
    let pass = std::env::var("PASS");
    let user = std::env::var("USER");
    let file_writer = FileManager::new("Users");
    println!("{:?}", imap_handler::search("Amy newman"));
    let search = match imap_handler::search("Amy newman") {
        Ok(s) => s,
        Err(_) => todo!(),
    };
    if pass.is_ok() && user.is_ok() {
        let user = user.unwrap();
        let pass = pass.unwrap();
        let mut manager = SmtpConnectionManager::new("smtp.gmail.com", &user, &pass);
        let new_email = Email::new(
            "example.lmao@example.com",
            "Bob Ross",
            "New mail",
            "I sent you this mail.",
        );
        new_email.send_email(&mut manager);
        for result in search {
            file_writer.write_file(&result);
        }
        let email_body = fetch_inbox_top();
        match email_body {
            Ok(b) => println!("{}", b.unwrap()),
            Err(_) => println!("Error!"),
        }
    }
    app::run();
}

#[allow(dead_code)]
fn fetch_inbox_top() -> imap::error::Result<Option<String>> {
    let domain = "imap.gmail.com";
    let tls = native_tls::TlsConnector::builder().build().unwrap();

    // we pass in the domain twice to check that the server's TLS
    // certificate is valid for the domain we're connecting to.
    let client = imap::connect((domain, 993), domain, &tls).unwrap();

    // the client we have here is unauthenticated.
    // to do anything useful with the e-mails, we need to log in
    let pass = std::env::var("PASS").unwrap();
    let user = std::env::var("USER").unwrap();
    let mut imap_session = client.login(user, pass).map_err(|e| e.0)?;

    // we want to fetch the first email in the INBOX mailbox
    imap_session.select("INBOX")?;

    // fetch message number 1 in this mailbox, along with its RFC822 field.
    // RFC 822 dictates the format of the body of e-mails
    let messages = imap_session.fetch("1", "RFC822")?;
    let message = if let Some(m) = messages.iter().next() {
        m
    } else {
        return Ok(None);
    };

    // extract the message's body
    let body = message.body().expect("message did not have a body!");
    let body = std::str::from_utf8(body)
        .expect("message was not valid utf-8")
        .to_string();

    // be nice to the server and log out
    imap_session.logout()?;

    Ok(Some(body))
}
