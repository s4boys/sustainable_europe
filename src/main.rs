use serde::{Serialize, Deserialize};
use std::error::Error;

use actix_web::{web, App, HttpResponse, HttpServer, Responder};


// Example
// Goal, Target, Indicator, SeriesCode, SeriesDescription, GeoAreaCode, GeoAreaName, TimePeriod, Value, Time_Detail,  UpperBound, LowerBound, BasePeriod, Source, FootNote, Nature, Units,[Reporting Type]
// "6","6.3","6.3.2","EN_H2O_OPAMBQ","Proportion of open water bodies with good ambient water quality (%)","40","Austria","2017","91.94","2013-2015","","","","Environment Live","","C","PERCENT","G"


#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "PascalCase")] 
struct Record {
    goal: String,
    target: String,
    indicator: String,
    series_code: String,
    series_description: String,
    geo_area_code: String,
    geo_area_name: String,
    time_period: String,
    value: f64,
    time_detail: String,
    upper_bound: String,
    lower_bound: String,
    base_period: String,
    source: String,
    foot_note: String,
    nature: String,
    units: String,
    reporting_type: Option<String>,
}

struct AppState {
    records_as_json: String,
}

fn main() {

    let rec = match parse_data() {
        Ok(r) => r,
        Err(_e) => Vec::new(),
    };

    let as_json = serde_json::to_string(&rec).unwrap();
    // records.iter().for_each(|r| println!("{:?}: {:?}", r.geo_area_name, r.value))

    HttpServer::new(move || {
        App::new()
            .data(AppState {
                records_as_json: as_json.clone(),
            })
            .route("/", web::get().to(index))
            .route("/data", web::get().to(data))
    })
    .bind("127.0.0.1:8088")
    .unwrap()
    .run()
    .unwrap();
}


// CSV parsing
fn parse_data() -> Result<Vec<Record>, Box<dyn Error>> {
    let mut rdr = csv::Reader::from_path("./data/target6_3fixed.csv")?;
    let mut result = Vec::new();
    
    for r in rdr.deserialize() {
        let record: Record = r?;
        result.push(record)

    }
    Ok(result)
}


// HTTP Handlers
fn index() -> impl Responder {
    HttpResponse::Ok().body("Welcome to the server. Visit /data to receive the parsed UN data.")
}

fn data(data: web::Data<AppState>) -> impl Responder {
    HttpResponse::Ok().body(&data.records_as_json)
}