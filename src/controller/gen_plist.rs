use axum::{
    body::Body,
    extract::Query,
    http::{StatusCode, header},
    response::Response,
};

use crate::data::plist_query::PlistQuery;

pub async fn get_gen_plist(Query(plist_query): Query<PlistQuery>) -> Response {
    let plist_xml = plist_query.build_template();

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/x-plist")
        .body(Body::from(plist_xml))
        .unwrap()
}
