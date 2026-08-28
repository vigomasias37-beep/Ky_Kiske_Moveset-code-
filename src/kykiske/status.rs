use {
    super::*,
    smash::{
        app::{lua_bind::*, sv_animcmd::*, *},
        hash40,
        lib::lua_const::*,
        lua2cpp::*,
        phx::*,
    },
    smash_script::*,
    smashline::*,
};

use super::*;

pub mod nspecial;

pub fn install() {

    nspecial::install();


}