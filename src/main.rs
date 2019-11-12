use serde::Deserialize;
use std::error::Error;


// Example
// Goal, Target, Indicator, SeriesCode, SeriesDescription, GeoAreaCode, GeoAreaName, TimePeriod, Value, Time_Detail,  UpperBound, LowerBound, BasePeriod, Source, FootNote, Nature, Units,[Reporting Type]
// "6","6.3","6.3.2","EN_H2O_OPAMBQ","Proportion of open water bodies with good ambient water quality (%)","40","Austria","2017","91.94","2013-2015","","","","Environment Live","","C","PERCENT","G"


#[derive(Debug, Deserialize)]
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
    time_Detail: String,
    upper_bound: String,
    lower_bound: String,
    base_period: String,
    source: String,
    foot_note: String,
    nature: String,
    units: String,
    reporting_type: Option<String>,
}



fn main() {

    match parse_data() {
        Ok(records) => records.iter().for_each(|r| println!("{:?}: {:?}", r.geo_area_name, r.value)),
        Err(e) => {
            println!("error parsing data: {}", e);
            std::process::exit(1);
        }
    }

}


fn parse_data() -> Result<Vec<Record>, Box<dyn Error>> {
    // let mut rdr = csv::Reader::from_reader(std::io::stdin());
    let mut rdr = csv::Reader::from_path("./data/target6_3fixed.csv")?;
    let mut result = Vec::new();
    
    for r in rdr.deserialize() {
        let record: Record = r?;
        result.push(record)

        // println!("{:?}", record);
    }
    Ok(result)
}