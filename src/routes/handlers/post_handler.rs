use std::path::PathBuf;

use crate::utils::{self, api_response, api_state, jwt::Claims};
use actix_multipart::form::{tempfile::TempFile, text::Text, MultipartForm};
use actix_web::{get, post, web};
use chrono::{NaiveDateTime, Utc};
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set, TransactionTrait};
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(MultipartForm)]
struct CreatePostModel {
    text: Text<String>,
    title: Text<String>,
    file: TempFile,
}
#[derive(Serialize, Deserialize, Debug)]
struct PostModel {
    pub id: i32,
    pub title: String,
    pub text: String,
    pub uuid: Uuid,
    pub image: Option<String>,
    pub user_id: i32,
    pub created_at: NaiveDateTime,
    pub user: Option<UserModel>,
}
#[derive(Serialize, Deserialize, Debug)]
struct UserModel {
    email: String,
    name: String,
}

#[post("create")]
pub async fn create_post(
    app_state: web::Data<api_state::AppState>,
    claim_data: Claims,
    post_data: MultipartForm<CreatePostModel>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let check_name = post_data
        .file
        .file_name
        .clone()
        .unwrap_or("null".to_owned());
    let max_file_size = (*utils::constants::MAX_FILE_SIZE).clone();

    match &check_name[check_name.len() - 4..] {
        ".png" | ".jpg" => {}
        _ => {
            return Err(api_response::ApiResponse::new(
                400,
                "Invalid file type".to_owned(),
            ))
        }
    }

    match post_data.file.size {
        0 => {
            return Err(api_response::ApiResponse::new(
                400,
                "File size is too small".to_owned(),
            ))
        }
        size if size > max_file_size as usize => {
            return Err(api_response::ApiResponse::new(
                401,
                "File size is too large".to_owned(),
            ))
        }
        _ => {}
    }
    let posts_entity = entity::post::ActiveModel {
        text: Set(post_data.text.clone()),
        title: Set(post_data.title.clone()),
        uuid: Set(Uuid::new_v4()),
        user_id: Set(claim_data.id),
        created_at: Set(Utc::now().naive_local()),
        ..Default::default()
    };
    let txn = app_state
        .db
        .begin()
        .await
        .map_err(|err| api_response::ApiResponse::new(400, err.to_string()))?;
    let mut create_entity = posts_entity
        .save(&txn)
        .await
        .map_err(|err| api_response::ApiResponse::new(400, err.to_string()))?;
    let temp_file_path = post_data.file.file.path();
    let file_name = post_data
        .file
        .file_name
        .as_ref()
        .map(|m| m.as_ref())
        .unwrap_or("null");
    let mut file_path = PathBuf::from("./public");
    let new_file_name = format!("{}.{}", Utc::now().timestamp(), file_name);

    file_path.push(new_file_name.clone());
    match std::fs::copy(temp_file_path, file_path) {
        Ok(_) => {
            create_entity.image = Set(Some(new_file_name));
            create_entity
                .save(&txn)
                .await
                .map_err(|err| api_response::ApiResponse::new(400, err.to_string()))?;
            txn.commit()
                .await
                .map_err(|err| api_response::ApiResponse::new(400, err.to_string()))?;
            std::fs::remove_file(temp_file_path).unwrap_or_default();
            Ok(api_response::ApiResponse::new(
                200,
                "Post Created".to_owned(),
            ))
        }

        Err(_) => {
            txn.rollback()
                .await
                .map_err(|err| api_response::ApiResponse::new(400, err.to_string()))?;

            Err(api_response::ApiResponse::new(
                400,
                "Error copying file".to_owned(),
            ))
        }
    }
}

#[get("my-post")]
pub async fn get_my_post(
    app_state: web::Data<api_state::AppState>,
    claim_data: Claims,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let post_data: Vec<PostModel> = entity::post::Entity::find()
        .filter(entity::post::Column::UserId.eq(claim_data.id))
        .all(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .into_iter()
        .map(|post| PostModel {
            id: post.id,
            title: post.title,
            text: post.text,
            uuid: post.uuid,
            image: post.image,
            user_id: post.user_id,
            created_at: post.created_at,
            user: None,
        })
        .collect();
    let res_str = serde_json::to_string(&post_data).unwrap();
    Ok(api_response::ApiResponse::new(200, res_str.to_owned()))
}

#[get("all-post")]
pub async fn get_all_post(
    app_state: web::Data<api_state::AppState>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let post_data: Vec<PostModel> = entity::post::Entity::find()
        .all(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .into_iter()
        .map(|post| PostModel {
            id: post.id,
            title: post.title,
            text: post.text,
            uuid: post.uuid,
            image: post.image,
            user_id: post.user_id,
            created_at: post.created_at,
            user: None,
        })
        .collect();
    let res_str = serde_json::to_string(&post_data).unwrap();
    Ok(api_response::ApiResponse::new(200, res_str.to_owned()))
}

#[get("post/{post_uuid}")]
pub async fn get_one_post(
    app_state: web::Data<api_state::AppState>,
    post_uuid: web::Path<Uuid>,
) -> Result<api_response::ApiResponse, api_response::ApiResponse> {
    let post_data: PostModel = entity::post::Entity::find()
        .filter(entity::post::Column::Uuid.eq(post_uuid.clone()))
        .find_also_related(entity::user::Entity)
        .one(&app_state.db)
        .await
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?
        .map(|post| PostModel {
            id: post.0.id,
            title: post.0.title,
            text: post.0.text,
            uuid: post.0.uuid,
            image: post.0.image,
            user_id: post.0.user_id,
            created_at: post.0.created_at,
            user: post.1.map(|user| UserModel {
                name: user.name,
                email: user.email,
            }),
        })
        .ok_or(api_response::ApiResponse::new(
            404,
            "Post not found".to_owned(),
        ))?;
    let res_str = serde_json::to_string(&post_data)
        .map_err(|err| api_response::ApiResponse::new(500, err.to_string()))?;
    Ok(api_response::ApiResponse::new(200, res_str.to_owned()))
}
