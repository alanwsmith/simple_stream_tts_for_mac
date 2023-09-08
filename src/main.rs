use std::process::Command;
use twitch_irc::login::StaticLoginCredentials;
use twitch_irc::ClientConfig;
use twitch_irc::SecureTCPTransport;
use twitch_irc::TwitchIRCClient;

#[tokio::main]
pub async fn main() {
    let config = ClientConfig::default();
    let (mut incoming_messages, client) =
        TwitchIRCClient::<SecureTCPTransport, StaticLoginCredentials>::new(config);
    let join_handle = tokio::spawn(async move {
        while let Some(message) = incoming_messages.recv().await {
            match message {
                twitch_irc::message::ServerMessage::Privmsg(payload) => {
                    if payload.message_text.starts_with("!talk") {
                        dbg!("asdfadsf");
                        let _ = Command::new("/usr/bin/say")
                            .args(vec![
                                "-v",
                                "Karen",
                                payload.message_text.replace("!talk", "").as_str(),
                            ])
                            .output()
                            .unwrap();
                    }
                }
                _ => {}
            }
        }
    });
    client.join("thebotofalan".to_owned()).unwrap();
    // client.join("theidofalan".to_owned()).unwrap();
    join_handle.await.unwrap();
}
