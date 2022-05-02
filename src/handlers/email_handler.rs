extern crate lettre;
extern crate lettre_email;

use lettre::{smtp::authentication::Credentials, SmtpClient, SmtpTransport, Transport};
use lettre_email::EmailBuilder;
pub struct SmtpConnectionManager {
    pub email_addr: String,
    pub mailer: SmtpTransport,
}

impl SmtpConnectionManager {
    pub fn new(domain: &str, email_addr: &str, password: &str) -> Self {
        let creds = Credentials::new(email_addr.to_owned(), password.to_owned());
        let client = SmtpClient::new_simple(domain).unwrap();
        let client = SmtpClient::credentials(client, creds);

        SmtpConnectionManager {
            email_addr: email_addr.to_string(),
            mailer: SmtpTransport::new(client),
        }
    }
}

pub struct Email {
    receiver_addr: String,
    receiver_name: String,
    subject: String,
    content: String,
}

impl Email {
    pub fn new(receiver_addr: &str, receiver_name: &str, subject: &str, body: &str) -> Self {
        Email {
            receiver_addr: receiver_addr.to_string(),
            receiver_name: receiver_name.to_string(),
            subject: subject.to_string(),
            content: body.to_string(),
        }
    }
    pub fn send_email(&self, connection_manager: &mut SmtpConnectionManager) {
        let email = EmailBuilder::new()
            .to((self.receiver_addr.clone(), self.receiver_name.clone()))
            .from(connection_manager.email_addr.clone())
            .subject(self.subject.clone())
            .text(self.content.clone())
            .build()
            .unwrap();

        // Send the email
        let result = connection_manager.mailer.send(email.into());

        if result.is_ok() {
            println!("Email sent");
        } else {
            println!("Could not send email: {:?}", result);
        }

        assert!(result.is_ok());
    }
}

#[derive(Debug)]
pub struct Account {
    email_addr: String,
    password: String,
    imap_domain: String,
    smtp_domain: String,
}
impl Account {
    pub fn new(
        email_addr: String,
        password: String,
        imap_domain: String,
        smtp_domain: String,
    ) -> Self {
        Account {
            email_addr,
            password,
            imap_domain,
            smtp_domain,
        }
    }

    fn fetch_inbox_top(&self) -> imap::error::Result<Option<String>> {
        let domain = &self.imap_domain;
        let tls = native_tls::TlsConnector::builder().build().unwrap();

        // we pass in the domain twice to check that the server's TLS
        // certificate is valid for the domain we're connecting to.
        let client = imap::connect((domain.as_str(), 993), domain, &tls).unwrap();

        // the client we have here is unauthenticated.
        // to do anything useful with the e-mails, we need to log in
        // let pass = std::env::var("PASS").unwrap();
        // let user = std::env::var("USER").unwrap();
        let email = &self.email_addr;
        let pass = &self.password;
        let mut imap_session = client.login(&email, &pass).map_err(|e| e.0)?;

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
}
