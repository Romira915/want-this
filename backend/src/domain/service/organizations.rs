use actix_session::Session;
use actix_web::{
    get,
    http::header,
    post,
    web::{self, Data},
    HttpRequest, HttpResponse, Result,
};
use api_format::Organization as OrganizationAPI;
use reqwest::StatusCode;

use crate::{
    domain::{
        entity::organizations::{JoinOrganization, JoinRequestOrganization},
        repositories::organizations::{MySqlOrganizationsRepository, OrganizationsRepository},
    },
    session::SessionKey,
    utility::get_user_id,
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
            return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
        }
    };

    let joined_org_list = match orgs_repo.fetch_joined_orgs(user_id).await {
        Ok(joined_org_list) => joined_org_list,
        Err(e) => {
            log::error!("{:?}", &e);
            return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
        }
    };
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

    Ok(HttpResponse::Ok().json(&org_list))
}

#[post("/organizations/{organization_id}")]
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
            log::error!("{}", &e);
            return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
        }
    };

    Ok(HttpResponse::Ok().finish())
}
