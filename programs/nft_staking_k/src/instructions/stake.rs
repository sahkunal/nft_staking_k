use anchor_lang::prelude ::*;
use mpl_core::{
    ID as MPL_CORE_ID,
    accounts::{BaseAssetV1, BaseCollectionV1},
    instructions::{AddPluginV1CpiBuilder, UpdatePluginV1Builder},
    types::{UpdateAuthority, Attribute, Attributes, Plugin, PluginAuthority, PluginType, FreezeDelegate},
    fetch_plugin,
};
use crate ::state::Config;
use crate:: error::ErrorCode;

#[derive(Accounts)]

pub struct Stake<'info>{
    #[account(mut)]
    pub owner:Signer<'info>,
    #[account(
        seeds=[b"config", collection.key().as_ref()],
        bump = config.bumps,
    )]
    pub config: Account<'info,Config>,
    #[account(
        mut,
        has_one= owner @ ErrorCode::InvalidOwner,
        constraint= asset.update_authority== UpdateAuthority:: Collection(collection.key()) @ ErrorCode:: InvalidUpdateAuthority,
    )]
    pub asset:Account<'info, BaseAssetV1>,
    #[account(
        mut,
        has_one= update_authority @ ErrorCode:: InvalidUpdateAuthority,
    )]
    pub collection:Account<'info, BaseCollectionV1>,
    ///CHECK
    #[account(
        seeds= [b"update_authority", collection.key().as_ref()],
        bump,
    )]
    pub update_authority: UncheckedAccount<'info>,
    pub system_program: Program<'info, System>,
    /// CHECK
    #[account(address= MPL_CORE_ID)]
    pub mpl_core_program: UncheckedAccount<'info>,
}

    pub fn handler(ctx: Context<Stake>)-> Result<()>{
        let attributes_fetched: Option<Attributes>= fetch_plugin::<BaseAssetV1, Attributes>(
            &ctx.accounts.asset.to_account_info(),
            PluginType::Attributes,
        )
        .ok()
        .map(|(_,attrs,_)|attrs);
    let mut attributes_list: Vec<Attribute>= Vec::new();

    if let Some(attributes)= &attributes_fetched{
        for attribute in &attributes.attribute_list{
            if attribute.key== "staked"{
                require!(attribute.value== "false", ErrorCode:: AlreadyStaked);
        }
        else if attribute.key!="staked_at"{
            attributes_list.push(attribute.clone());
    }
}
    }
    //add the staking attributes
    attributes_list.push(Attribute { key: "staked".to_string(),
})
    }

