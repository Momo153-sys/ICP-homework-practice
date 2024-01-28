use candid::{CandidType, Decode, Deserialize, Encode};
use ic_stable_structures::memory_manager::{MemoryId, MemoryManager, VirtualMemory};
use ic_stable_structures::{BoundedStorable, DefaultMemoryImpl, StableBTreeMap, Storable};
use reqwest::{Error, Response};
use std::clone;
use std::{borrow::Cow, cell::RefCell}; 
use ic_cdk::api::management_canister::http_request::{
    http_request, CanisterHttpRequestArgument, HttpMethod,
};
use near_sdk::serde_json;
use chrono::{Date, DateTime, Local,NaiveDate,Duration,Utc};


type Memory = VirtualMemory<DefaultMemoryImpl>;
const MAX_VALUE_SIZE: u32 = 100;

#[derive(CandidType,Deserialize,Clone)]
struct NasaData {
    url:String,
    start_date: String,
    end_date: String,
    api_key:String,
    thumbs:bool,
}


impl Storable for NasaData {
    fn to_bytes(&self) -> Cow<[u8]> {
        Cow::Owned(Encode!(self).unwrap())
    }

    fn from_bytes(bytes: Cow<[u8]>) -> Self {
        Decode!(bytes.as_ref(), Self).unwrap()
    }
}
impl BoundedStorable for NasaData {
    const MAX_SIZE: u32 = MAX_VALUE_SIZE; // Adjust the size as needed
    const IS_FIXED_SIZE: bool = false;
}
thread_local! {
    static MEMORY_MANAGER: RefCell<MemoryManager<DefaultMemoryImpl>> =
    RefCell::new(MemoryManager::init(DefaultMemoryImpl::default()));

    static DATA_MAP: RefCell<StableBTreeMap<u64, NasaData, Memory>> = RefCell::new(
        StableBTreeMap::init(
            MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))), // Use a different MemoryId if needed
        )
    );
}



#[ic_cdk::query]
fn get_data_link(api_key:String) -> String{
   let url = "https://api.nasa.gov/planetary/apod";
   let final_url = format!("{}?api_key={}",url,api_key);
   final_url
}


#[ic_cdk::update]
async fn get_data_from_api() -> String {
    // Setup the URL for the HTTP GET request
    let url = "https://api.nasa.gov/planetary/apod?api_key=JwEttTpTs4nr2uRxVpvxLeQgw3niK4NVWsxoKfyL
    ".to_string();

    // Prepare headers for the system http_request call
    let request_headers = vec![];

    // Setup the HTTP request arguments
    let request = CanisterHttpRequestArgument {
        url,
        method: HttpMethod::GET,
        body: None,
        max_response_bytes: None,
        transform: None,
        headers: request_headers,
    };


    let cycles:u128 =1;
let final_request = http_request(request,cycles).await;
let status_success :u128 =200;


    // Make the HTTPS request and wait for the response
    match final_request {
        Ok((response,)) => {
            if response.status == status_success {
                // Parse the JSON response into a Vec<Event>
                let events: Vec<NasaData> =
                    serde_json::from_slice(&response.body).expect("Failed to parse JSON response.");

                DATA_MAP.with(|events_map_ref| {
                    let mut events_map = events_map_ref.borrow_mut();
                    // Create a new map and fill it with the new events
                    let mut new_map = StableBTreeMap::init(
                        MEMORY_MANAGER.with(|m| m.borrow().get(MemoryId::new(1))),
                    );
                    for (i, event) in events.into_iter().enumerate() {
                        new_map.insert(i as u64, event);
                    }
                    // Replace the old map with the new one
                    *events_map = new_map;
                });
                // Return a success message
                "Events fetched and stored successfully.".to_string()
            } else {
                format!("HTTP request failed with status code: {}", response.status)
            }
        }
        Err((code, message)) => {
            format!(
                "The http_request resulted in an error. Code: {:?}, Message: {}",
                code, message
            )
        }
    }
}