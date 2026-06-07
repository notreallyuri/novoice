use crate::core::{crypto, error::AppError, sessions, state::SharedState};
use entity::{user, user_account, user_settings};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use shared::dtos::auth::RegisterRequest;
use uuid::Uuid;

pub async fn register(state: SharedState, payload: RegisterRequest) -> Result<String, AppError> {
    let existing_user = user_account::Entity::find()
        .filter(entity::user_account::Column::Email.eq(&payload.account.email))
        .one(&state.db)
        .await?;

    if existing_user.is_some() {
        return Err(AppError::BadRequest("Email is already in use".into()));
    }

    let existing_username = user::Entity::find()
        .filter(user::Column::Username.eq(&payload.account.username))
        .one(&state.db)
        .await?;

    if existing_username.is_some() {
        return Err(AppError::Conflict("Username is already taken".into()));
    }

    let hashed_password = crypto::hash_password(&payload.account.password)?;
    let new_user_id = Uuid::new_v4();

    let now = chrono::Utc::now().into();

    let txn = state.db.begin().await?;

    let new_user = user::ActiveModel {
        id: Set(new_user_id),
        username: Set(payload.account.username),
        display_name: Set(payload.profile.display_name),
        avatar_url: Set(payload.profile.avatar_url),
        banner_url: Set(payload.profile.banner_url),
        bio: Set(payload.profile.bio),
        profile_color: Set(None),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let new_account = user_account::ActiveModel {
        user_id: Set(new_user_id),
        email: Set(payload.account.email),
        password_hash: Set(hashed_password),
        verified: Set(false),
        two_factor_enabled: Set(false),
        two_factor_secret: Set(None),
        recovery_keys: Set(vec![]),
        created_at: Set(now),
        updated_at: Set(now),
    };

    let new_settings = user_settings::ActiveModel {
        user_id: Set(new_user_id),
        theme_dark_mode: Set(entity::user_settings::DbThemeDarkMode::System),
        theme_color: Set(entity::user_settings::DbThemeColor::Default),
        theme_rounding: Set(entity::user_settings::DbThemeRounding::Default),
        theme_spacing: Set(entity::user_settings::DbThemeSpacing::Default),
        notification_active: Set(true),
    };

    new_user.insert(&txn).await?;
    new_account.insert(&txn).await?;
    new_settings.insert(&txn).await?;

    txn.commit().await?;

    let token = sessions::create(&state.redis.sessions, &new_user_id.to_string()).await?;

    Ok(token)
}
