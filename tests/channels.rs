mod utils;

use assert_matches::assert_matches;

use discord_next::model::{Channel, ChannelType, PublicChannel};
use discord_next::Error;

use self::utils::{random_str, Environment};

#[test]
fn text_public_create_list_delete() -> Result<(), Error> {
    let env = Environment::connect();
    let name = random_str();

    let channel =
        env.discord
            .create_channel(env.server, &name, ChannelType::Text)?;

    let public = match channel {
        Channel::Public(p) => p,
        other => panic!("expected public channel, got `{:?}`", other),
    };

    assert_eq!(public.name, name);
    assert_eq!(public.server_id, env.server);
    assert_eq!(public.kind, ChannelType::Text);
    assert_eq!(public.bitrate, None);
    assert_eq!(public.nsfw, false);

    env.discord
        .get_server_channels(env.server)?
        .into_iter()
        .find(|c| c.name == name)
        .unwrap();

    let deleted = env.discord.delete_channel(public.id)?;

    assert_matches!(
        deleted,
        Channel::Public(PublicChannel { name: del, .. }) if del == name
    );

    assert!(env
        .discord
        .get_server_channels(env.server)?
        .into_iter()
        .find(|c| c.name == name)
        .is_none());

    Ok(())
}

#[test]
fn text_public_edit_get() -> Result<(), Error> {
    let env = Environment::connect();
    let name = random_str();
    let topic = random_str();

    let channel = env.discord.edit_channel(env.text_channel, |edit| {
        edit.name(&name).topic(&topic)
    })?;

    assert_eq!(channel.id, env.text_channel);
    assert_eq!(channel.topic.as_ref(), Some(&topic));
    assert_eq!(channel.name, name);

    let got = match env.discord.get_channel(env.text_channel)? {
        Channel::Public(p) => p,
        other => panic!("expected public channel, got `{:?}`", other),
    };

    assert_eq!(got.id, env.text_channel);
    assert_eq!(got.topic, Some(topic));
    assert_eq!(got.name, name);

    Ok(())
}

#[test]
fn text_public_create_get_delete_invite() -> Result<(), Error> {
    let env = Environment::connect();

    let invite = env.discord.create_invite(env.text_channel, 500, 3, false)?;

    assert_eq!(invite.max_age, 500);
    assert_eq!(invite.max_uses, 3);
    assert_eq!(invite.temporary, false);
    assert_eq!(invite.server_id, env.server);

    let got = env.discord.get_invite(&invite.code)?;

    assert_eq!(got.server_id, env.server);
    assert_eq!(got.channel_id, env.text_channel);
    assert_eq!(got.channel_type, ChannelType::Text);
    assert_eq!(got.code, invite.code);

    let deleted = env.discord.delete_invite(&invite.code)?;

    assert_eq!(deleted.server_id, env.server);
    assert_eq!(deleted.channel_id, env.text_channel);
    assert_eq!(deleted.channel_type, ChannelType::Text);
    assert_eq!(deleted.code, invite.code);

    env.discord.get_invite(&invite.code).unwrap_err();

    Ok(())
}
