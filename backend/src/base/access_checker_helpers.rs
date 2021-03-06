use crate::base::http::ServiceError;
use crate::base::request::{Identity, RequestContext};

pub fn reject_unauthorized(ctx: &RequestContext) -> Result<(), ServiceError> {
    if let Identity::User(_) = ctx.identity {
        return Ok(());
    }
    if let Identity::Service(_) = ctx.identity {
        return Ok(());
    }
    Err(ServiceError::unauthorized())
}
