use {
    smash::{
        lua2cpp::*,
        phx::*,
        app::{sv_animcmd::*, lua_bind::*, *},
        lib::{lua_const::*, L2CValue, L2CAgent},
    },
    smash_script::*,
    smashline::*
};

unsafe extern "C" fn ike_sound_win1(agent: &mut L2CAgentBase) {
    if WorkModule::get_int(agent.module_accessor, *FIGHTER_INSTANCE_WORK_ID_INT_COLOR) % 2 == 0 {
        frame(agent.lua_state_agent, 15.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ike_win01"));
        }
    frame(agent.lua_state_agent, 38.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ike_special_s03"));
    }
    frame(agent.lua_state_agent, 71.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ike_special_s03"));
    }
    frame(agent.lua_state_agent, 122.0);
    if macros::is_excute(agent) {
        macros::PLAY_SE(agent, Hash40::new("se_ike_squat"));
    }
}
    else {
        frame(agent.lua_state_agent, 38.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_special_s03"));
        }
        frame(agent.lua_state_agent, 45.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE_NO_3D(agent, Hash40::new("vc_ike_win01"));
        }
        frame(agent.lua_state_agent, 71.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_special_s03"));
        }
        frame(agent.lua_state_agent, 122.0);
        if macros::is_excute(agent) {
            macros::PLAY_SE(agent, Hash40::new("se_ike_squat"));
        }
                      }
                     }

pub fn install() {
    Agent::new("ike")
     .sound_acmd("sound_win1",ike_sound_win1)
     .install();
}