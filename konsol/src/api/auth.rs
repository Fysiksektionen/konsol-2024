use crate::models::{AuthenticatedUser, PermissionLevel, User};
use leptos::prelude::*;

#[server(prefix = "/konsol/api")]
pub async fn verify_google_credential(id_token: String) -> Result<AuthenticatedUser, ServerFnError> {
    use crate::actions;
    use crate::db;
    use google_oauth::AsyncClient;

    let client_id =
        std::env::var("GOOGLE_ID_TOKEN").map_err(|_| ServerFnError::new("GOOGLE_ID_TOKEN not set"))?;

    let client = AsyncClient::new(&client_id);
    let payload = client
        .validate_id_token(&id_token)
        .await
        .map_err(|_| ServerFnError::new("Invalid Google ID token"))?;

    let email = payload
        .email
        .ok_or_else(|| ServerFnError::new("No email in Google payload"))?;

    let mut conn = db::get_conn()?;
    let email_for_lookup = email.clone();
    let permission = tokio::task::spawn_blocking(move || actions::check_user(&mut conn, &email_for_lookup))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .ok_or_else(|| ServerFnError::new("Not an authorized user"))?;

    let user = AuthenticatedUser { email, permission };
    crate::session::set_authenticated_user(&user).await?;
    log::info!("User {} authenticated", user.email);
    Ok(user)
}

#[server(prefix = "/konsol/api")]
pub async fn auth_status() -> Result<AuthenticatedUser, ServerFnError> {
    crate::session::require_auth().await
}

/// The Google OAuth client ID, needed client-side by the Google Identity
/// Services button. Same value `verify_google_credential` validates the
/// token's audience against, so it's read from the same env var rather than
/// a second one — unlike a Vite build, there's no build-time bundling step
/// to bake a `VITE_`-prefixed variable into the client JS, so this has to
/// cross the client/server boundary at request time instead.
#[server(prefix = "/konsol/api")]
pub async fn get_google_client_id() -> Result<String, ServerFnError> {
    std::env::var("GOOGLE_ID_TOKEN").map_err(|_| ServerFnError::new("GOOGLE_ID_TOKEN not set"))
}

#[server(prefix = "/konsol/api")]
pub async fn logout() -> Result<(), ServerFnError> {
    crate::session::require_auth().await?;
    crate::session::clear_authenticated_user();
    Ok(())
}

#[server(prefix = "/konsol/api")]
pub async fn add_user(email: String, permission: PermissionLevel) -> Result<User, ServerFnError> {
    use crate::actions;
    use crate::db;
    use uuid::Uuid;

    crate::session::require_admin().await?;

    let mut conn = db::get_conn()?;
    let user = User {
        id: Uuid::new_v4().to_string(),
        email,
        admin: permission == PermissionLevel::Admin,
    };
    tokio::task::spawn_blocking(move || actions::insert_user(&mut conn, user))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(prefix = "/konsol/api")]
pub async fn remove_user(id: String) -> Result<(), ServerFnError> {
    use crate::actions;
    use crate::db;

    crate::session::require_admin().await?;

    let mut conn = db::get_conn()?;
    tokio::task::spawn_blocking(move || actions::remove_user(&mut conn, &id))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .map_err(|e| ServerFnError::new(e.to_string()))
}

#[server(prefix = "/konsol/api")]
pub async fn list_users() -> Result<Vec<User>, ServerFnError> {
    use crate::actions;
    use crate::db;

    crate::session::require_admin().await?;

    let mut conn = db::get_conn()?;
    tokio::task::spawn_blocking(move || actions::get_all_users(&mut conn))
        .await
        .map_err(|e| ServerFnError::new(e.to_string()))?
        .map_err(|e| ServerFnError::new(e.to_string()))
}
