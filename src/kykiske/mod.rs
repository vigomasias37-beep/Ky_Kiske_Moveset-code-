#![allow(unused_macros, unused_imports, unused_variables, static_mut_refs, unused_unsafe)]

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

pub mod acmd;
pub mod opff;
pub mod status;

pub fn install() {
    unsafe {}

    acmd::install();
    status::install();
    opff::install();
}
