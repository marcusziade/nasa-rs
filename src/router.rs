use worker::*;
use crate::{handlers, middleware};

#[allow(dead_code)]
pub type HandlerContext = (Env, Context);

#[allow(dead_code)]
pub fn create_router(env: Env, ctx: Context) -> Router<'static, HandlerContext> {
    Router::with_data((env, ctx))
        // Landing page - serve the HTML
        .get_async("/", |_, _| async { 
            let html = include_str!("../docs/index.html");
            Response::from_html(html)
        })
        
        // Serve CSS
        .get_async("/styles.css", |_, _| async {
            let css = include_str!("../docs/styles.css");
            Response::ok(css)
                .map(|mut res| {
                    res.headers_mut()
                        .set("Content-Type", "text/css")
                        .unwrap();
                    res
                })
        })
        
        // Serve favicon
        .get_async("/favicon.svg", |_, _| async {
            let svg = include_str!("../docs/favicon.svg");
            Response::ok(svg)
                .map(|mut res| {
                    res.headers_mut()
                        .set("Content-Type", "image/svg+xml")
                        .unwrap();
                    res
                })
        })
        
        // Health check
        .get_async("/health", |_, _| async { 
            Response::ok("OK")
        })
        
        // APOD (Astronomy Picture of the Day)
        .get_async("/api/apod", handlers::apod::get_apod)
        
        // NeoWs (Near Earth Objects)
        .get_async("/api/neo/feed", handlers::neo::get_neo_feed)
        .get_async("/api/neo/:asteroid_id", handlers::neo::get_neo_lookup)
        .get_async("/api/neo/browse", handlers::neo::get_neo_browse)
        
        // DONKI (Space Weather)
        .get_async("/api/donki/cme", handlers::donki::get_cme)
        .get_async("/api/donki/cme-analysis", handlers::donki::get_cme_analysis)
        .get_async("/api/donki/gst", handlers::donki::get_gst)
        .get_async("/api/donki/ips", handlers::donki::get_ips)
        .get_async("/api/donki/flr", handlers::donki::get_flr)
        .get_async("/api/donki/sep", handlers::donki::get_sep)
        .get_async("/api/donki/mpc", handlers::donki::get_mpc)
        .get_async("/api/donki/rbe", handlers::donki::get_rbe)
        .get_async("/api/donki/hss", handlers::donki::get_hss)
        .get_async("/api/donki/wsa-enlil", handlers::donki::get_wsa_enlil)
        .get_async("/api/donki/notifications", handlers::donki::get_notifications)
        
        // Earth Imagery
        .get_async("/api/earth/imagery", handlers::earth::get_imagery)
        .get_async("/api/earth/assets", handlers::earth::get_assets)
        
        // EPIC
        .get_async("/api/epic/natural/all", handlers::epic::get_natural_all)
        .get_async("/api/epic/natural/date/:date", handlers::epic::get_natural_date)
        .get_async("/api/epic/enhanced/all", handlers::epic::get_enhanced_all)
        .get_async("/api/epic/enhanced/date/:date", handlers::epic::get_enhanced_date)
        
        // Mars Rover Photos
        .get_async("/api/mars-photos/:rover/photos", handlers::mars::get_rover_photos)
        .get_async("/api/mars-photos/:rover/latest", handlers::mars::get_latest_photos)
        .get_async("/api/mars-photos/manifests/:rover", handlers::mars::get_manifest)
        
        // Tech Transfer
        .get_async("/api/techtransfer/patents", handlers::tech::get_patents)
        .get_async("/api/techtransfer/patents-issued", handlers::tech::get_patents_issued)
        .get_async("/api/techtransfer/software", handlers::tech::get_software)
        .get_async("/api/techtransfer/spinoffs", handlers::tech::get_spinoffs)
        
        // NASA Image and Video Library
        .get_async("/api/media/search", handlers::media::search_media)
        .get_async("/api/media/asset/:nasa_id", handlers::media::get_asset)
        .get_async("/api/media/metadata/:nasa_id", handlers::media::get_metadata)
        .get_async("/api/media/captions/:nasa_id", handlers::media::get_captions)
        
        // Exoplanet Archive
        .get_async("/api/exoplanets/query", handlers::exoplanets::query_exoplanets)
        
        // SSD/CNEOS (Solar System Dynamics)
        .get_async("/api/ssd/cad", handlers::ssd::get_close_approach)
        .get_async("/api/ssd/sbdb", handlers::ssd::get_small_body)
        .get_async("/api/ssd/sentry", handlers::ssd::get_sentry)
        .get_async("/api/ssd/scout", handlers::ssd::get_scout)
        .get_async("/api/ssd/nhats", handlers::ssd::get_nhats)
        .get_async("/api/ssd/fireballs", handlers::ssd::get_fireballs)
        
        // Apply CORS middleware to all routes
        .or_else_any_method_async("/*catchall", |req, _| async move {
            middleware::cors::handle_cors(req).await
        })
}