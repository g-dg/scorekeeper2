use axum::http::StatusCode;

#[derive(Debug)]
pub struct GenericError(u16);

impl GenericError {
    pub const BAD_REQUEST: GenericError = GenericError(400);
    pub const FORBIDDEN: GenericError = GenericError(403);
    pub const NOT_FOUND: GenericError = GenericError(404);
    pub const CONFLICT: GenericError = GenericError(409);
    pub const INTERNAL_SERVER_ERROR: GenericError = GenericError(500);

    pub fn to_status_code(&self) -> StatusCode {
        StatusCode::from_u16(self.0).unwrap()
    }
}
