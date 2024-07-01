use weather::WeatherApi;

mod weather;

#[tokio::main]
async fn main() {
    // let json = r#"
    //     [
    //       {
    //         "Version": 1,
    //         "Key": "216189",
    //         "Type": "City",
    //         "Rank": 31,
    //         "LocalizedName": "Firenze",
    //         "Country": {
    //           "ID": "IT",
    //           "LocalizedName": "Italia"
    //         },
    //         "AdministrativeArea": {
    //           "ID": "52",
    //           "LocalizedName": "Toscana"
    //         }
    //       },
    //       {
    //         "Version": 1,
    //         "Key": "2201236",
    //         "Type": "City",
    //         "Rank": 65,
    //         "LocalizedName": "Firestone",
    //         "Country": {
    //           "ID": "US",
    //           "LocalizedName": "Stati Uniti"
    //         },
    //         "AdministrativeArea": {
    //           "ID": "CO",
    //           "LocalizedName": "Colorado"
    //         }
    //       },
    //       {
    //         "Version": 1,
    //         "Key": "216052",
    //         "Type": "City",
    //         "Rank": 85,
    //         "LocalizedName": "Firenzuola",
    //         "Country": {
    //           "ID": "IT",
    //           "LocalizedName": "Italia"
    //         },
    //         "AdministrativeArea": {
    //           "ID": "52",
    //           "LocalizedName": "Toscana"
    //         }
    //       },
    //       {
    //         "Version": 1,
    //         "Key": "2564470",
    //         "Type": "City",
    //         "Rank": 85,
    //         "LocalizedName": "Firenzuola",
    //         "Country": {
    //           "ID": "IT",
    //           "LocalizedName": "Italia"
    //         },
    //         "AdministrativeArea": {
    //           "ID": "55",
    //           "LocalizedName": "Umbria"
    //         }
    //       },
    //       {
    //         "Version": 1,
    //         "Key": "215892",
    //         "Type": "City",
    //         "Rank": 95,
    //         "LocalizedName": "Firenze Nova",
    //         "Country": {
    //           "ID": "IT",
    //           "LocalizedName": "Italia"
    //         },
    //         "AdministrativeArea": {
    //           "ID": "52",
    //           "LocalizedName": "Toscana"
    //         }
    //       }
    //     ]
    // "#;

    let client = WeatherApi::new("", Some(String::from("it")));
    // let cities = client.autocomplete("fir").await.unwrap();
    // println!("{:?}", cities);

    let hours = client.hourly12("216189").await.unwrap();
    println!("{:?}", hours);
}
