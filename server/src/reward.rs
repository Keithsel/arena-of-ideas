use player_tag::player_tag;

use super::*;

#[spacetimedb::table(public, name = reward)]
#[derive(Default)]
pub struct TReward {
    #[primary_key]
    pub id: u64,
    #[index(btree)]
    pub owner: u64,
    pub source: String,
    pub bundle: ItemBundle,
}

impl TReward {
    fn new(source: String, bundle: ItemBundle) -> Self {
        Self {
            source,
            bundle,
            ..default()
        }
    }
    fn add(mut self, ctx: &ReducerContext, owner: u64) {
        self.owner = owner;
        self.id = next_id(ctx);
        ctx.db.reward().insert(self);
    }
    pub fn daily(ctx: &ReducerContext) {
        for player in ctx
            .db
            .player_tag()
            .iter()
            .filter(|t| PlayerTag::from_str(&t.tag).unwrap().is_supporter())
            .map(|t| t.owner)
        {
            Self::new(
                "Supporter Reward".into(),
                TLootboxItem::new(ctx, 0, LootboxKind::Regular).into(),
            )
            .add(ctx, player);
        }
    }
}

#[spacetimedb::reducer]
fn reward_claim(ctx: &ReducerContext, id: u64) -> Result<(), String> {
    let player = ctx.player()?;
    let reward: TReward = ctx
        .db
        .reward()
        .id()
        .find(id)
        .context_str("Reward not found")?;
    player.check_owner(reward.owner)?;
    reward.bundle.take(ctx, player.id)?;
    ctx.db.reward().id().delete(id);
    Ok(())
}
