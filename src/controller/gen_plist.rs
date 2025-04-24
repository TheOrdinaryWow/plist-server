use std::collections::HashMap;

use axum::{
    body::Body,
    extract::Query,
    http::{StatusCode, header},
    response::Response,
};

use crate::data::plist_query::PlistQuery;

pub async fn get_gen_plist(Query(params): Query<HashMap<String, String>>) -> Response {
    let plist_query = PlistQuery {
        bundleid: params.get("bundleid").map(|s| s.to_string()).unwrap_or_default(),
        version: params.get("version").map(|s| s.to_string()).unwrap_or_default(),
        name: params.get("name").map(|s| s.to_string()).unwrap_or_default(),
        fetchurl: params.get("fetchurl").map(|s| s.to_string()).unwrap_or_default(),
    };

    let plist_xml = plist_query.build_template();

    Response::builder()
        .status(StatusCode::OK)
        .header(header::CONTENT_TYPE, "application/octet-stream")
        .body(Body::from(plist_xml))
        .unwrap()
}
