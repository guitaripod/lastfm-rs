use worker::*;

mod common;
pub mod error;
mod handlers;
pub mod middleware;
pub mod models;
mod utils;

#[cfg(not(target_arch = "wasm32"))]
pub mod cli;

// Re-export for tests
#[cfg(test)]
pub use models::sign_request;

use handlers::{album, artist, auth, chart, geo, library, tag, track, user};
use serde_json::Value;

#[event(fetch)]
async fn main(req: Request, env: Env, _ctx: Context) -> Result<Response> {
    console_error_panic_hook::set_once();

    let router = Router::new();

    router
        // Health check
        .get("/health", |_, _| Response::ok("OK"))
        // API Documentation endpoints
        .get("/api/docs", |_, _| {
            let swagger_ui = include_str!("swagger-ui.html");
            Response::ok(swagger_ui)
                .map(|mut resp| {
                    resp.headers_mut()
                        .set("Content-Type", "text/html; charset=utf-8")
                        .unwrap();
                    resp
                })
        })
        .get("/api/docs/openapi.yaml", |_, _| {
            let openapi_spec = include_str!("../openapi.yaml");
            Response::ok(openapi_spec)
                .map(|mut resp| {
                    resp.headers_mut()
                        .set("Content-Type", "application/x-yaml")
                        .unwrap();
                    resp
                })
        })
        .get("/api/docs/openapi.json", |_, _| {
            let openapi_yaml = include_str!("../openapi.yaml");
            // Convert YAML to JSON
            match serde_yaml::from_str::<Value>(openapi_yaml) {
                Ok(yaml_value) => {
                    match serde_json::to_string_pretty(&yaml_value) {
                        Ok(json_string) => Response::ok(json_string)
                            .map(|mut resp| {
                                resp.headers_mut()
                                    .set("Content-Type", "application/json")
                                    .unwrap();
                                resp
                            }),
                        Err(_) => Response::error("Failed to convert to JSON", 500)
                    }
                },
                Err(_) => Response::error("Failed to parse OpenAPI spec", 500)
            }
        })
        // Legacy endpoint for backwards compatibility
        .get("/openapi", |req, _| {
            let target = req
                .url()?
                .join("/api/docs/openapi.yaml")
                .map_err(|e| worker::Error::from(e.to_string()))?;
            Response::redirect(target)
        })
        // Artist endpoints
        .get_async("/artist/getCorrection", artist::get_correction)
        .get_async("/artist/getInfo", artist::get_info)
        .get_async("/artist/getSimilar", artist::get_similar)
        .get_async("/artist/getTopAlbums", artist::get_top_albums)
        .get_async("/artist/getTopTags", artist::get_top_tags)
        .get_async("/artist/getTopTracks", artist::get_top_tracks)
        .get_async("/artist/search", artist::search)
        // Album endpoints
        .get_async("/album/getInfo", album::get_info)
        .get_async("/album/getTopTags", album::get_top_tags)
        .get_async("/album/search", album::search)
        // Track endpoints
        .get_async("/track/getCorrection", track::get_correction)
        .get_async("/track/getInfo", track::get_info)
        .get_async("/track/getSimilar", track::get_similar)
        .get_async("/track/getTopTags", track::get_top_tags)
        .get_async("/track/search", track::search)
        // Chart endpoints
        .get_async("/chart/getTopArtists", chart::get_top_artists)
        .get_async("/chart/getTopTags", chart::get_top_tags)
        .get_async("/chart/getTopTracks", chart::get_top_tracks)
        // Geo endpoints
        .get_async("/geo/getTopArtists", geo::get_top_artists)
        .get_async("/geo/getTopTracks", geo::get_top_tracks)
        // Tag endpoints
        .get_async("/tag/getInfo", tag::get_info)
        .get_async("/tag/getSimilar", tag::get_similar)
        .get_async("/tag/getTopAlbums", tag::get_top_albums)
        .get_async("/tag/getTopArtists", tag::get_top_artists)
        .get_async("/tag/getTopTags", tag::get_top_tags)
        .get_async("/tag/getTopTracks", tag::get_top_tracks)
        .get_async("/tag/getWeeklyChartList", tag::get_weekly_chart_list)
        // User endpoints (public methods only)
        .get_async("/user/getFriends", user::get_friends)
        .get_async("/user/getInfo", user::get_info)
        .get_async("/user/getLovedTracks", user::get_loved_tracks)
        .get_async("/user/getPersonalTags", user::get_personal_tags)
        .get_async("/user/getRecentTracks", user::get_recent_tracks)
        .get_async("/user/getTopAlbums", user::get_top_albums)
        .get_async("/user/getTopArtists", user::get_top_artists)
        .get_async("/user/getTopTags", user::get_top_tags)
        .get_async("/user/getTopTracks", user::get_top_tracks)
        .get_async("/user/getWeeklyAlbumChart", user::get_weekly_album_chart)
        .get_async("/user/getWeeklyArtistChart", user::get_weekly_artist_chart)
        .get_async("/user/getWeeklyChartList", user::get_weekly_chart_list)
        .get_async("/user/getWeeklyTrackChart", user::get_weekly_track_chart)
        // Library endpoints
        .get_async("/library/getArtists", library::get_artists)
        // Auth endpoints (require API secret)
        .get_async("/auth/getSession", auth::get_session)
        .get_async("/auth/getMobileSession", auth::get_mobile_session)
        .get_async("/auth/url", auth::get_auth_url)
        // Catch all for unmatched routes
        .or_else_any_method_async(
            "/:path",
            |_, _| async move { Response::error("Not Found", 404) },
        )
        .run(req, env)
        .await
        .or_else(|err| {
            console_error!("Router error: {}", err);
            Response::error("Internal Server Error", 500)
        })
}
