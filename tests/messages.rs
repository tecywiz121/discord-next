mod utils;

use discord_next::model::MessageType;
use discord_next::Error;

use self::utils::{random_str, Environment};

#[test]
fn send_edit_pin_get_delete() -> Result<(), Error> {
    let env = Environment::connect();
    let _conn = env.discord.connect();

    let text = random_str();
    let text2 = random_str();
    let nonce = random_str();

    let sent =
        env.discord
            .send_message(env.text_channel, &text, &nonce, false)?;

    assert_eq!(sent.channel_id, env.text_channel);
    assert_eq!(&sent.content, &text);
    assert_eq!(sent.nonce.as_ref(), Some(&nonce));
    assert_eq!(sent.tts, false);
    assert_eq!(sent.edited_timestamp, None);
    assert_eq!(sent.pinned, false);
    assert_eq!(sent.kind, MessageType::Regular);
    assert_eq!(sent.mention_everyone, false);
    assert!(sent.mentions.is_empty());
    assert!(sent.mention_roles.is_empty());
    assert!(sent.reactions.is_empty());
    assert!(sent.attachments.is_empty());
    assert!(sent.embeds.is_empty());

    let edited = env
        .discord
        .edit_message(env.text_channel, sent.id, &text2)?;

    assert_eq!(edited.channel_id, env.text_channel);
    assert_eq!(&edited.content, &text2);

    env.discord.pin_message(env.text_channel, sent.id)?;

    let got = env.discord.get_message(env.text_channel, sent.id)?;

    assert_eq!(got.channel_id, env.text_channel);
    assert_eq!(&got.content, &text2);
    assert_eq!(got.tts, false);
    assert!(got.edited_timestamp.is_some());
    assert_eq!(got.pinned, true);
    assert_eq!(got.kind, MessageType::Regular);
    assert_eq!(got.mention_everyone, false);
    assert!(got.mentions.is_empty());
    assert!(got.mention_roles.is_empty());
    assert!(got.reactions.is_empty());
    assert!(got.attachments.is_empty());
    assert!(got.embeds.is_empty());

    env.discord.delete_message(env.text_channel, sent.id)?;

    Ok(())
}
