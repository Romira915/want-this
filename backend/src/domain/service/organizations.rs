use actix_session::Session;
use actix_web::{get, http::header, web::Data, HttpRequest, HttpResponse, Result};
use reqwest::StatusCode;

use crate::{
    domain::repositories::organizations::{MySqlOrganizationsRepository, OrganizationsRepository},
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
        return Ok(HttpResponse::SeeOther()
            .insert_header((header::LOCATION, format!("{}/", CONFIG.frontend_origin)))
            .finish());
    }

    let org_list = match orgs_repo.fetch_public_orgs().await {
        Ok(org_list) => org_list,
        Err(e) => {
            log::error!("{:?}", &e);
            return Ok(HttpResponse::build(StatusCode::UNAUTHORIZED).finish());
        }
    };

    Ok(HttpResponse::Ok().json(&org_list))
}
