use ges_cli::ges_protos::ges_proto_service_client::GesProtoServiceClient;
use ges_cli::ges_protos::{SendEmailRequest, SendEmailResponse};


//Make this single threaded.  We don't need multi thread here, it's just for sending
//simple emails and if we need more than one thread, we are doing it wrong.
//Tonic is heavily integrated with tokio, otherwise, I wouldn't be using it at all
#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Connecting to personal cli service...");
    let mut connect = GesProtoServiceClient::connect("http://localhost:50055").await?;
    println!("Connected!");

    let email_response = connect.send_email(SendEmailRequest {emails: vec!["a".to_string(), "b".to_string(), "c".to_string()], subject: "subject".to_string(), body: "body".to_string()}).await?;
    println!("{:?}", email_response);

    Ok(())

}
