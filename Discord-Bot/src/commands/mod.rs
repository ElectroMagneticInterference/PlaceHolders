mod general;
mod owners;

use general::*;
use owners::*;

use serenity::framework::standard::macros::group;

#[group]
#[commands(ping, echo)]
struct General;

#[group]
#[prefixes("owners", "o")]
#[commands(quit)]
struct Owners;
