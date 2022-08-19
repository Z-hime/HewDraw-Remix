// opff import
utils::import_noreturn!(common::opff::fighter_common_opff);
use super::*;
use globals::*;

 
unsafe fn dragon_fang_shot_dash_cancel(boma: &mut BattleObjectModuleAccessor, status_kind: i32, situation_kind: i32, cat1: i32, frame: f32) {
    if status_kind == *FIGHTER_KAMUI_STATUS_KIND_SPECIAL_N_SHOOT && frame > 7.0 {
        if situation_kind == *SITUATION_KIND_GROUND {
            if boma.is_cat_flag(Cat1::Dash) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_DASH, false);
            }
            if boma.is_cat_flag(Cat1::TurnDash) {
                StatusModule::change_status_request_from_script(boma, *FIGHTER_STATUS_KIND_TURN_DASH, false);
            }
        }
    }
}

unsafe fn dragon_surge(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, motion_kind: u64, frame: f32) {
    if motion_kind == hash40("special_lw") {
        if frame == 1.0 {
            MotionModule::change_motion(boma, Hash40::new("special_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
            ArticleModule::change_motion(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("special_lw_hit"), true, 0.0);
        }
    } else if motion_kind == hash40("special_air_lw") {
        if frame == 1.0 {
            MotionModule::change_motion(boma, Hash40::new("special_air_lw_hit"), 0.0, 1.0, false, 0.0, false, false);
            ArticleModule::change_motion(boma, *FIGHTER_KAMUI_GENERATE_ARTICLE_WATERDRAGON, Hash40::new("special_air_lw_hit"), true, 0.0);
        }
    }
}

pub unsafe fn moveset(fighter: &mut L2CFighterCommon, boma: &mut BattleObjectModuleAccessor, id: usize, cat: [i32 ; 4], status_kind: i32, situation_kind: i32, motion_kind: u64, stick_x: f32, stick_y: f32, facing: f32, frame: f32) {
    //dragon_fang_shot_dash_cancel(boma, status_kind, situation_kind, cat[0], frame);
    //dragon_surge(fighter, boma, motion_kind, frame);
}

#[utils::macros::opff(FIGHTER_KIND_KAMUI )]
pub fn kamui_frame_wrapper(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    unsafe {
        common::opff::fighter_common_opff(fighter);
		kamui_frame(fighter)
    }
}

pub unsafe fn kamui_frame(fighter: &mut smash::lua2cpp::L2CFighterCommon) {
    if let Some(info) = FrameInfo::update_and_get(fighter) {
        moveset(fighter, &mut *info.boma, info.id, info.cat, info.status_kind, info.situation_kind, info.motion_kind.hash, info.stick_x, info.stick_y, info.facing, info.frame);
    }
}