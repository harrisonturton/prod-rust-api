use crate::util::access_checker_helpers::reject_unauthorized;
use crate::util::http::ServiceError;
use crate::util::request::RequestContext;

pub fn can_access_list_users(ctx: &RequestContext) -> Result<(), ServiceError> {
    reject_unauthorized(ctx)
}

pub fn can_access_find_user(ctx: &RequestContext) -> Result<(), ServiceError> {
    reject_unauthorized(ctx)
}

pub fn can_access_create_user(_: &RequestContext) -> Result<(), ServiceError> {
    Ok(())
}
