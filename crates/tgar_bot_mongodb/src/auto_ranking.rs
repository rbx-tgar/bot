use tgar_bot_utils::ROBLOX_CLIENT;

use crate::models::player::Player;

pub struct Rank {
    pub role_id: u64,
    pub id: u64,
    pub required_xp: i16,
}

const GROUP: u64 = 9326891;
const RANKS: [Rank; 10] = [
    Rank { role_id: 56440143, id: 10, required_xp: 0 },
    Rank { role_id: 56445169, id: 20, required_xp: 2 },
    Rank { role_id: 56445177, id: 30, required_xp: 5 },
    Rank { role_id: 56445194, id: 40, required_xp: 10 },
    Rank { role_id: 56445584, id: 50, required_xp: 18 },
    Rank { role_id: 56445598, id: 60, required_xp: 30 },
    Rank { role_id: 56445669, id: 70, required_xp: 50 },
    Rank { role_id: 56445687, id: 80, required_xp: 80 },
    Rank { role_id: 91973787, id: 81, required_xp: 120 },
    Rank { role_id: 89630317, id: 90, required_xp: 200 },
];

fn get_xp_rank(xp: i16) -> Option<&'static Rank> {
    RANKS.iter().filter(|r| r.required_xp <= xp).max_by_key(|r| r.required_xp)
}

pub async fn try_rank(user_id: u64) {
    let player = Player::from_user_id(user_id as i64).await;
    let current_rank =
        ROBLOX_CLIENT.get_group_member_rank(user_id, GROUP).await.expect("roblox api is down");

    if !RANKS.iter().any(|r| r.id == current_rank) {
        return;
    }

    if let Some(rank) = get_xp_rank(player.xp) {
        ROBLOX_CLIENT
            .set_group_member_role(user_id, GROUP, rank.role_id)
            .await
            .expect("unable to rank player");
    }
}
