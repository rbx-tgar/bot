use mongodb::bson::doc;
use tgar_bot_utils::{mongo_client, serde_enum, serde_struct};

//<editor-fold desc="player medals macro" defaultState="collapsed">
#[macro_export]
macro_rules! player_medals {
    (
        $(
            $variant:ident => { name: $n:expr, desc: $d:expr }
        ),* $(,)?
    ) => {
        #[derive(serde::Serialize, serde::Deserialize, Clone, Debug, Eq, PartialEq)]
        pub enum Medal {
            $(
                $variant
            ),*
        }

        impl Medal {
            /// The display name of the medal
            pub fn name(&self) -> &'static str {
                match self {
                    $(Self::$variant => $n),*
                }
            }

            /// The medal's description
            pub fn description(&self) -> &'static str {
                match self {
                    $(Self::$variant => $d),*
                }
            }
        }
    };
}
//</editor-fold>

player_medals!(
    // assignable medals
    Contribution => {
        name: "Medal of Contribution",
        desc: "This medal is awarded to those who've significantly contributed to the group as a developer or staff member."
    },
    Activity => {
        name: "Medal of Activity",
        desc: "This medal is awarded to dedicated officers who've shown immense activity and have successfully hosted a plethora of events."
    },
    VCHonor => {
        name: "Mas Amedda's Medal of Honor",
        desc: "This exceedingly rare medal is awarded by the Vice Chancellor to whoever they deem worthy."
    },
    SCHonor => {
        name: "Sheev Palpatine's Medal of Honor",
        desc: "This exceedingly rare medal is awarded by the Supreme Chancellor to whoever they deem worthy."
    },
    // automatic metals
    BronzeStar => {
        name: "Bronze Star",
        desc: "This medal is awarded to those who have 300 experience points or higher."
    },
    SilverStar => {
        name: "Silver Star",
        desc: "This medal is awarded to those who have 500 experience points or higher."
    },
    GoldStar => {
        name: "Gold Star",
        desc: "This medal is awarded to those who have 800 experience points or higher."
    },
    DiamondStar => {
        name: "Diamond Star",
        desc: "This medal is awarded to those who have 1,000 experience points or higher."
    },
    PlatinumStar => {
        name: "Platinum Star",
        desc: "This medal is awarded to those who have 1,300 experience points or higher."
    },
);

serde_enum!(StatusType {
    /// Indicates that the player should be protected by the Red Guard
    VIP,
    /// Indicates that the player is arrest-on-sight
    AOS,
    /// Indicates that the player is kill-on-sight
    KOS,
    /// No status
    None,
});

serde_struct!(Status {
    /// The reason this status was assigned
    reason: String,
    /// The person who assigned the status to this player
    #[serde(rename = "assignedBy")]
    assigned_by: String,
    /// The type of this status
    kind: StatusType,
});

serde_struct!(Player {
    /// The unique identifier for the Player assigned by roblox
    id: i64,
    /// The amount of experience points the player has
    xp: i16,
    /// The amount of credits the player has
    credits: i64,
    /// The medals that have been awarded to the player
    medals: Vec<Medal>,
    /// The assigned status of the player
    status: Option<Status>,
});

impl Player {
    pub async fn from_user_id_strict(id: i64) -> Option<Self> {
        let client = mongo_client().await;
        let collection = client.database("game").collection::<Player>("players");

        let filter = doc! { "id": id };
        let player = collection.find_one(filter).await;

        match player {
            Ok(Some(p)) => Some(p),
            Ok(None) => None,
            Err(_) => None, // suppressed
        }
    }

    pub async fn from_user_id(id: i64) -> Self {
        match Self::from_user_id_strict(id).await {
            Some(p) => p,
            None => {
                let player = Player { id, xp: 0, credits: 0, medals: vec![], status: None };

                let client = mongo_client().await;
                let collection = client.database("game").collection::<Player>("players");

                collection
                    .insert_one(player.clone())
                    .await
                    .expect("couldn't store new player inside db");

                player
            }
        }
    }
}
