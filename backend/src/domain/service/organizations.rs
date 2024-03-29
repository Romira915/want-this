use std::str::FromStr;

use actix_session::Session;
use actix_web::{
    delete, get,
    http::header::{self, ContentType},
    post, put,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};
use api_format::Organization as OrganizationAPI;
use api_format::User as UserAPI;
use reqwest::StatusCode;

use crate::{
    domain::{
        entity::organizations::{JoinOrganization, JoinRequestOrganization, JoinStatus},
        repositories::organizations::{MySqlOrganizationsRepository, OrganizationsRepository},
    },
    session::SessionKey,
    utility::{get_user_id, get_user_id_unchecked, is_login},
    CONFIG,
};

#[get("/organizations")]
async fn get_not_joined_organizations(
    _req: HttpRequest,
    session: Session,
    orgs_repo: Data<MySqlOrganizationsRepository>,
) -> Result<HttpResponse> {
    let user_id = if let Some(user_id) = get_user_id(&session)? {
        user_id
    } else {
        return Ok(HttpResponse::NotFound()
            .insert_header(("WantThis-Location", format!("{}/", CONFIG.frontend_origin)))
            .finish());
    };

    let org_list = match orgs_repo.fetch_public_orgs().await {
        Ok(org_list) => org_list,
        Err(e) => {
            log::error!("{:?}", &e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    let joined_org_list = match orgs_repo.fetch_joined_orgs(user_id).await {
        Ok(joined_org_list) => joined_org_list,
        Err(e) => {
            log::error!("{:?}", &e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    // NOTE: 公開組織から加入済み組織を除去
    let org_list = org_list.into_iter().filter(|org| {
        for joined in &joined_org_list {
            if org.organization_id == joined.organization_id {
                return false;
            }
        }
        true
    });

    let org_list: Vec<_> = org_list
        .into_iter()
        .map(|o| OrganizationAPI::from(o))
        .collect();
    log::debug!("{:#?}", org_list);

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(&org_list))
}

/// NOTE: 加入リクエスト一覧を取得
#[get("/organizations/{organization_id}/join_request")]
async fn get_join_request_list(
    _req: HttpRequest,
    path: web::Path<u64>,
    session: Session,
    orgs_repo: Data<MySqlOrganizationsRepository>,
) -> Result<HttpResponse> {
    let _user_id = if let Some(user_id) = get_user_id(&session)? {
        user_id
    } else {
        return Ok(HttpResponse::NotFound()
            .insert_header(("WantThis-Location", format!("{}/", CONFIG.frontend_origin)))
            .finish());
    };

    let org_id = path.into_inner();

    let request_pending_users = match orgs_repo.fetch_join_request_is_pending_users(org_id).await {
        Ok(users) => users,
        Err(e) => {
            log::error!("{:?}", &e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    let request_pending_users: Vec<_> = request_pending_users
        .into_iter()
        .map(|u| UserAPI::from(u))
        .collect();

    Ok(HttpResponse::Ok()
        .content_type(ContentType::json())
        .json(&request_pending_users))
}

#[post("/organizations/{organization_id}/join_request")]
async fn join_request_organizations(
    _req: HttpRequest,
    path: web::Path<u64>,
    session: Session,
    orgs_repo: Data<MySqlOrganizationsRepository>,
) -> Result<HttpResponse> {
    let user_id = if let Some(user_id) = get_user_id(&session)? {
        user_id
    } else {
        return Ok(HttpResponse::NotFound()
            .insert_header(("WantThis-Location", format!("{}/", CONFIG.frontend_origin)))
            .finish());
    };

    let org_id = path.into_inner();
    let join_req_org = JoinRequestOrganization::new(user_id, org_id, false);

    let id = match orgs_repo.join_request_organization(&join_req_org).await {
        Ok(id) => id,
        Err(e) => {
            log::error!("{:?}", &e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    Ok(HttpResponse::Ok().finish())
}

/// TODO: 編集権限持ちのみが実行可能にする
#[put("/organizations/{organization_id}")]
async fn update_organizations(
    _req: HttpRequest,
    path: web::Path<u64>,
    update_org: web::Json<OrganizationAPI>,
    session: Session,
    orgs_repo: Data<MySqlOrganizationsRepository>,
) -> Result<HttpResponse> {
    let org_id = path.into_inner();
    if org_id
        != update_org
            .organization_id
            .parse::<u64>()
            .expect("Failed to parse() organization_id")
    {
        log::info!("pathとorganization_idが不一致");
        return Ok(HttpResponse::build(StatusCode::BAD_REQUEST).finish());
    }

    let user_id = if let Some(user_id) = get_user_id_unchecked(&session) {
        user_id
    } else {
        return Ok(HttpResponse::NotFound()
            .insert_header(("WantThis-Location", format!("{}/", CONFIG.frontend_origin)))
            .finish());
    };

    let org = match orgs_repo.find_org_by_org_id(org_id).await {
        Ok(org) => org,
        Err(e) => {
            log::error!("{:?}", &e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    // NOTE: オーナーと編集権限持ちは実行
    // doneTODO: 編集権限持ちも実行可能にする
    if user_id == org.owner
        || orgs_repo
            .fetch_edit_permission(user_id, org_id)
            .await
            .unwrap_or(None)
            .unwrap_or(false)
    {
        let update_org = update_org.into_inner();

        // NOTE: 組織情報更新
        if let Err(e) = orgs_repo.update_org(&update_org).await {
            log::error!("{:?}", &e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    }

    Ok(HttpResponse::Ok().finish())
}

/// 加入リクエストを処理するAPI
#[put("/organizations/{organization_id}/join_request/{user_id}/{join_status}")]
async fn process_join_request_user(
    _req: HttpRequest,
    path: web::Path<(u64, u64, String)>,
    session: Session,
    orgs_repo: Data<MySqlOrganizationsRepository>,
) -> Result<HttpResponse> {
    let user_id = if let Some(user_id) = get_user_id_unchecked(&session) {
        user_id
    } else {
        return Ok(HttpResponse::NotFound()
            .insert_header(("WantThis-Location", format!("{}/", CONFIG.frontend_origin)))
            .finish());
    };

    let (org_id, join_request_user_id, update_join_status) = path.into_inner();
    let update_join_status =
        JoinStatus::from_str(&update_join_status).unwrap_or(JoinStatus::Pending);
    if update_join_status == JoinStatus::Pending {
        return Ok(HttpResponse::BadRequest().finish());
    }

    let org = match orgs_repo.find_org_by_org_id(org_id).await {
        Ok(org) => org,
        Err(e) => {
            log::error!("{:?}", &e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    // NOTE: オーナーと編集権限持ちは実行
    if user_id == org.owner
        || orgs_repo
            .fetch_edit_permission(user_id, org_id)
            .await
            .unwrap_or(None)
            .unwrap_or(false)
    {
        if let Err(e) = orgs_repo
            .update_join_status(join_request_user_id, org_id, &update_join_status)
            .await
        {
            log::error!("{:?}", &e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
        log::info!(
            "Update join_request user_id: {}; org_id: {}; join_status: {:?}",
            join_request_user_id,
            org_id,
            &update_join_status
        );
    }

    Ok(HttpResponse::Ok().finish())
}

#[delete("/organizations/{organization_id}")]
async fn delete_organizations(
    _req: HttpRequest,
    path: web::Path<u64>,
    session: Session,
    orgs_repo: Data<MySqlOrganizationsRepository>,
) -> Result<HttpResponse> {
    let user_id = if let Some(user_id) = get_user_id_unchecked(&session) {
        user_id
    } else {
        return Ok(HttpResponse::NotFound()
            .insert_header(("WantThis-Location", format!("{}/", CONFIG.frontend_origin)))
            .finish());
    };

    let org_id = path.into_inner();

    // NOTE: 組織が存在するか
    let org = match orgs_repo.find_org_by_org_id(org_id).await {
        Ok(org) => org,
        Err(e) => {
            log::error!("{:?}", &e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    // NOTE: オーナーのみ削除可能
    if org.owner == user_id {
        if let Err(e) = orgs_repo.delete_org(org_id).await {
            log::error!("{:?}", &e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    } else {
        log::warn!(
            "不当なユーザーから削除リクエスト． request user_id {}; org.owner {}",
            user_id,
            org.owner
        );
    }

    Ok(HttpResponse::Ok().finish())
}

#[delete("/organizations/{organization_id}/{delete_user_id}")]
async fn delete_user_from_organization(
    _req: HttpRequest,
    path: web::Path<(u64, u64)>,
    session: Session,
    orgs_repo: Data<MySqlOrganizationsRepository>,
) -> Result<HttpResponse> {
    let user_id = if let Some(user_id) = get_user_id_unchecked(&session) {
        user_id
    } else {
        return Ok(HttpResponse::NotFound()
            .insert_header(("WantThis-Location", format!("{}/", CONFIG.frontend_origin)))
            .finish());
    };

    let (org_id, delete_user_id) = path.into_inner();

    let org = match orgs_repo.find_org_by_org_id(org_id).await {
        Ok(org) => org,
        Err(e) => {
            log::error!("{:?}", &e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    };

    // NOTE: オーナーを削除することはできない
    if delete_user_id == org.owner {
        log::info!(
            "オーナーを削除することはできない． delete_user_id {}; owner {}",
            delete_user_id,
            org.owner
        );
        return Ok(HttpResponse::BadRequest().finish());
    }

    // NOTE: オーナーのみ実行する
    if user_id == org.owner {
        // TODO: 組織からユーザ削除
        if let Err(e) = orgs_repo
            .delete_user_from_organization(org_id, delete_user_id)
            .await
        {
            log::error!("{:?}", &e);
            return Ok(HttpResponse::InternalServerError().finish());
        }
    }

    Ok(HttpResponse::Ok().finish())
}
