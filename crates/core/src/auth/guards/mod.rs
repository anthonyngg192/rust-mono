use actix_web::guard::{Guard, GuardContext};

pub struct APIGuard;

impl Guard for APIGuard {
    fn check(&self, _ctx: &GuardContext<'_>) -> bool {
        true
    }
}
