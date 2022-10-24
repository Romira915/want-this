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
        entity::organizations::JoinOrganization,
        repositories::organizations::{MySqlOrganizationsRepository, OrganizationsRepository},
    },
    session::SessionKey,
    utility::is_login,
    CONFIG,
};

#[get("/organizations")]
async fn get_organizations(
    _req: HttpRequest,
    session: Session,
    orgs_repo: Data<MySqlOrganizationsRepository>,
) -> Result<HttpResponse> {
    if !is_login(&session)? {
        return Ok(HttpResponse::NotFound()
            .insert_header(("WantThis-Location", format!("{}/", CONFIG.frontend_origin)))
            .finish());
    }

    let org_list = match orgs_repo.fetch_public_orgs().await {
        Ok(org_list) => org_list,
        Err(e) => {
            log::error!("{:?}", &e);
            return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
        }
    };
    let org_list: Vec<OrganizationAPI> = org_list
        .into_iter()
        .map(|o| OrganizationAPI::from(o))
        .collect();
    log::debug!("{:#?}", org_list);

    Ok(HttpResponse::Ok().json(&org_list))
}

#[post("/organizations/{organization_id}")]
async fn join_organizations(
    _req: HttpRequest,
    path: web::Path<u64>,
    session: Session,
    orgs_repo: Data<MySqlOrganizationsRepository>,
) -> Result<HttpResponse> {
    if !is_login(&session)? {
        return Ok(HttpResponse::NotFound()
            .insert_header(("WantThis-Location", format!("{}/", CONFIG.frontend_origin)))
            .finish());
    }

    let org_id = path.into_inner();
    let join_org = JoinOrganization::new(
        session.get(SessionKey::UserId.as_ref()).unwrap().unwrap(),
        org_id,
        false,
    );

    let id = match orgs_repo.join_organization(&join_org).await {
        Ok(id) => id,
        Err(e) => {
            log::error!("{}", &e);
            return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
        }
    };

    Ok(HttpResponse::Ok().finish())
}