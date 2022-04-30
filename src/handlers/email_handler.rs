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
