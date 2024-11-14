pub mod auth {
    extern crate kiteconnect;
    extern crate serde_json as json;
    use actix_web::{get, web, HttpResponse, Responder};
    use kiteconnect::connect::KiteConnect;
    use dotenv::dotenv;
    use std::env;
    use serde::{Deserialize, Serialize};


    #[get("/login")]
    pub async fn log_in() -> impl Responder {

        dotenv().ok();

        let kite_api_key = env::var("API_KEY").expect("API KEY is not set");

        let  kc = KiteConnect::new(&kite_api_key, "");

        let login_url = kc.login_url();




        HttpResponse::Found()
                     .append_header(("Location", login_url))
                     .finish()
    }

    #[get("/redirect")]
    pub async fn start_session(query: web::Query<LoginQuery>) -> impl Responder {
        dotenv().ok();

        let kite_api_key = env::var("API_KEY").expect("API KEY is not set");
        let kite_api_secret = env::var("API_SECRET").expect("API SECRET is not set");
        let request_token = &query.request_token;

        println!("Request token is: {}", request_token);

        let  mut kc = KiteConnect::new(&kite_api_key, "");
        let session = kc.generate_session(&request_token, &kite_api_secret);
        let mut access_token = String::from(" ");

        match session {
            Ok(value) => {
                
                let api_response: ApiResponse = serde_json::from_value(value).unwrap();

                if api_response.status == "success" {
                    match api_response.data  {
                        Some(data) => {

                            access_token = data.access_token;

                        },

                        None => {
                            println!("Failed to fetched api response data");
                        }
                    }
                }

                //HttpResponse::Ok().body("Fetched data successfylly")

            },

            Err(err) => {
                println!("Error in generating session: {:?}", err);

                //HttpResponse::Ok().body("Failed to generate session")
            }
        }

        println!("Access token is: {:?}", access_token);
        
        kc.set_access_token(&access_token);

        let profile = kc.profile();
        println!("My profile is: {:?}", profile);

        match profile {
            Ok(value) => {
                println!("Profile is: {:?}", value);
                HttpResponse::Ok().json(value)
            },

            Err(err) => {
                println!("Error is: {:?}", err);
                HttpResponse::InternalServerError().body("Failed to get profile maybe access token is not set")
            }
        }


    }

    #[derive(Deserialize)]
    struct LoginQuery {
        request_token: String,
    }

    #[derive(Deserialize)]
    struct ApiResponse {
        data: Option<LoginData>,
        status: String
    }

    #[derive(Deserialize, Serialize, Debug)]
    struct LoginData {
        access_token: String,
        api_key: String,
        avatar_url: Option<String>,
        broker: String,
        email: String,
        enctoken: String,
        exchanges: Vec<String>,
        login_time: String,
        meta: Option<LoginMeta>,
        order_types: Vec<String>,
        products: Vec<String>,
        public_token: String,
        refresh_token: String,
        user_id: String,
        user_name: String,
        user_shortname: String,
        user_type: String,
    }

    #[derive(Deserialize, Serialize, Debug)]
    struct LoginMeta {
        demat_consent: String,
    }
}
