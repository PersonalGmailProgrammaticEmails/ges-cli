use clap::Parser;
use ges_cli::ges_protos::ges_proto_service_client::GesProtoServiceClient;
use ges_cli::ges_protos::{SendEmailRequest};


#[derive(Parser, Debug)]
#[command(version, about, long_about = None)]
struct Args {
    /// Subject of the email
    #[arg(short, long)]
    subject: String,

    /// Recipients to send emails too, line deliminated
    #[arg(short, long)]
    recipient_file: String,

    /// Body file of the email
    #[arg(short, long)]
    body_file: String,
}



//Make this single threaded.  We don't need multi thread here, it's just for sending
//simple emails and if we need more than one thread, we are doing it wrong.
//Tonic is heavily integrated with tokio, otherwise, I wouldn't be using it at all
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {

    let args = Args::parse();
    let subject = args.subject;
    let recipient_file = args.recipient_file;
    let body_file = args.body_file;

    let recipients = std::fs::read_to_string(recipient_file)?.lines().map(String::from).collect::<Vec<String>>();
    let body = std::fs::read_to_string(body_file)?;

    println!("Connecting to personal cli service...");
    let mut connect = GesProtoServiceClient::connect("http://localhost:50055").await?;
    println!("Connected!");

    let email_response = connect.send_email(SendEmailRequest {emails: recipients, subject, body}).await?;
    println!("{:?}", email_response);

    Ok(())
}
