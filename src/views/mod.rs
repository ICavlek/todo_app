mod app;
use app::app_views_factory;

mod auth;
use auth::auth_views_factory;

mod to_do;
use to_do::to_do_views_factory;

use actix_web::web::ServiceConfig;

pub fn views_factory(app: &mut ServiceConfig) {
    app_views_factory(app);
    auth_views_factory(app);
    to_do_views_factory(app);
}
