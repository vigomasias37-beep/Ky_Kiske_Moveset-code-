use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::lua_const::*,
        hash40
	},
    smash_script::*,
    smashline::*
};

use smashline::Priority::*;




pub fn install() {

    let costume = &mut Vec::new();
    unsafe { 
        for i in 0..crate::MARKED_COLORS.len() {
            if crate::MARKED_COLORS[i] {
                costume.push(i);
            }
        }
    }

    Agent::new("ken")
    .set_costume(costume.to_vec())

    
    
        .install();

}