use poise::CreateReply;
use tgar_bot_utils::ROBLOX_CLIENT;
use tgar_rblx_api::users::UsernameUserDetails;

pub struct Data;
pub type Error = Box<dyn std::error::Error + Send + Sync>;
pub type Context<'a> = poise::Context<'a, Data, Error>;

const SERVER_ID: u64 = 800934264531058690;
const TESTING_SERVER_ID: u64 = 1289087483396489227;

const SHADOW_MODERATOR: u64 = 1346564691215781979;
const OFFICER: u64 = 1300289170580963348;

async fn check_guild_permissions(ctx: Context<'_>) -> Result<bool, Error> {
    let guild_id = match ctx.guild_id() {
        Some(id) => id.get(),
        None => {
            ctx.send(
                CreateReply::default()
                    .content("You can only use this command in a guild, sorry!")
                    .ephemeral(true),
            )
            .await?;
            return Err("attempted to execute a guild only command".into());
        }
    };

    if guild_id == TESTING_SERVER_ID {
        return Ok(true);
    }

    if guild_id != SERVER_ID {
        ctx.send(
            CreateReply::default()
                .content("You lack the permissions required to use this command, sorry!")
                .ephemeral(true),
        )
        .await?;
        return Err("attempted to run guild only command outside of the main server".into());
    }

    let member = ctx.author_member().await.unwrap();

    let has_permission =
        member.roles.contains(&SHADOW_MODERATOR.into()) || member.roles.contains(&OFFICER.into());
    if !has_permission {
        ctx.send(
            CreateReply::default()
                .content("You lack the permissions required to use this command, sorry!")
                .ephemeral(true),
        )
        .await?;
        return Err("insufficient permissions".into());
    }

    Ok(true)
}

async fn collect_user_ids(usernames: &Vec<String>) -> Vec<u64> {
    let details: Vec<UsernameUserDetails> = ROBLOX_CLIENT
        .username_user_details(usernames.clone(), false)
        .await
        .expect("roblox api is down");
    details.into_iter().map(|d| d.id).collect()
}

pub mod xp {
    use poise::command;
    use tgar_bot_mongodb::models::player::Player;
    use tgar_bot_utils::mongo_client;

    use crate::commands::{Context, Error, check_guild_permissions, collect_user_ids};

    /// Parent command for everything xp related
    #[command(slash_command, guild_only, subcommands("add", "remove", "set", "view"))]
    pub async fn xp(ctx: Context<'_>) -> Result<(), Error> {
        // impl note: no-op
        Ok(())
    }

    /// Adds XP to the specified users
    #[command(slash_command, check = "check_guild_permissions")]
    pub async fn add(
        ctx: Context<'_>,
        #[description = "The usernames of the people you wish to add xp to"] users: String,
        #[description = "The amount of XP you want to add"] amount: u64,
    ) -> Result<(), Error> {
        ctx.defer().await?;
        let user_names: Vec<String> = users.split(',').map(|s| s.trim().to_string()).collect();
        for id in collect_user_ids(&user_names).await {
            tgar_bot_mongodb::add_xp(mongo_client().await, id as i64, amount as i64).await?;
        }
        ctx.reply(format!("Successfully added {} XP to {} user(s)!", amount, user_names.len()))
            .await?;
        Ok(())
    }

    /// Removes XP from the specified users
    #[command(slash_command, check = "check_guild_permissions")]
    pub async fn remove(
        ctx: Context<'_>,
        #[description = "The usernames of the people you wish to take xp from"] users: String,
        #[description = "The amount of XP you want to remove"] amount: u64,
    ) -> Result<(), Error> {
        ctx.defer().await?;
        let user_names: Vec<String> = users.split(',').map(|s| s.trim().to_string()).collect();
        for id in collect_user_ids(&user_names).await {
            tgar_bot_mongodb::remove_xp(mongo_client().await, id as i64, amount as i64).await?;
        }
        ctx.reply(format!("Successfully removed {} XP from {} user(s)!", amount, user_names.len()))
            .await?;
        Ok(())
    }

    /// Sets the XP of the specified users
    #[command(slash_command, check = "check_guild_permissions")]
    pub async fn set(
        ctx: Context<'_>,
        #[description = "The usernames of the people you wish to set the XP of"] users: String,
        #[description = "The amount of XP you want them to have"] amount: u64,
    ) -> Result<(), Error> {
        ctx.defer().await?;
        let user_names: Vec<String> = users.split(',').map(|s| s.trim().to_string()).collect();
        for id in collect_user_ids(&user_names).await {
            tgar_bot_mongodb::set_xp(mongo_client().await, id as i64, amount as i64).await?;
        }
        ctx.reply(format!("Successfully set {} users XP to {}!", user_names.len(), amount)).await?;
        Ok(())
    }

    /// Views the XP that a player has
    #[command(slash_command)]
    pub async fn view(
        ctx: Context<'_>,
        #[description = "The username of the person you want to view the XP of"] user: String,
    ) -> Result<(), Error> {
        let data = Player::from_user_id(
            *collect_user_ids(&vec![user.clone()]).await.first().expect("couldn't find player")
                as i64,
        )
        .await;
        ctx.reply(format!("{} has {} XP", user, data.xp)).await.expect("discord api is down");
        Ok(())
    }
}
