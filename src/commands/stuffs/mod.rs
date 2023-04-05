pub mod ping;
pub mod status;

use crate::{Context, Error};
use ping::ping;
use status::status;

#[poise::command(prefix_command, slash_command, subcommands("ping", "status"))]
pub async fn stuffs(_ctx: Context<'_>, _arg: String) -> Result<(), Error> {
    return Ok(()); //we're not gonna respond anyway lmfao
}
