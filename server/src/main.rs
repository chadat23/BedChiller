use warp::Filter;
use serde::{Serialize, Deserialize};

#[derive(Deserialize, Serialize)]
struct DataPoint {
    time: i32,
    temp: i32,
}

#[derive(Deserialize, Serialize)]
struct RequestData {
    name: String,
    setpoint: Vec<DataPoint>,
}

#[tokio::main]
async fn main() {
    let setpoints_route = warp::post()
        .and(warp::path("setpoints"))
        .and(warp::body::json())
        .map(|setpoint: RequestData| {
            println!("Name: {}", setpoint.name);
            for datapoint in setpoint.setpoint {
                println!("DataPoint: ({}, {})", datapoint.time, datapoint.temp);
            }

            warp::reply::with_status("Data processed", warp::http::StatusCode::ACCEPTED)
        });

    // warp::serve(setpoints_route).run(([127, 0, 0, 1], 3030)).await;
    warp::serve(setpoints_route).run(([0, 0, 0, 0], 3030)).await;
}
