use rocket::route::Route;
mod lightbulb;
mod schedule;

pub fn routes() -> Vec<Route> {
    routes![
        lightbulb::create_lightbulb,
        lightbulb::get_all_lightbulb,
        lightbulb::get_lightbulb,
        lightbulb::update_lightbulb,
        lightbulb::delete_lightbulb,
        schedule::create_schedule,
        schedule::list_schedule,
        schedule::get_schedule,
        schedule::update_schedule,
        schedule::delete_schedule
    ]
}
