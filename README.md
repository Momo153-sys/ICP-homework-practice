# NasaData

The code defines a system for accessing Nasa APIs through links which contains the API key and give informations about Nasa Astronomy Picture Of the Day.

### Key Structures


  1. **NadaData**

    - `url`: A `String` representing the link of the API website.
    - `start_date`: A `String` representing the starting time of the data.
    - `end_date`: A `String` representing the ending time of the data.
    - `api_key` :A `String` representing the API key.
    - `thumbs` : A `bool` checking if the data is a video or not.


### Storable Implementations

-    `NasaData` structures implement the `Storable` trait, allowing them to be serialized and deserialized for storage.

### Memory Management

-   The code uses `VirtualMemory` and `StableBTreeMap` to manage events in a stable memory structure.

### Functions

1. **get_data_link(api_key:String)**

    - Get API key and return the link for the Nasa APIs page.

2. **get_data_from_api()**
    - Makes an HTTP request to an external API to fetch events.
    - Updates the stored events with the fetched data.

### HTTP Request Handling

-   The `get_data_from_api` function demonstrates how to make an HTTP GET request, handle the response, and update the stored events.


## Running the project locally

If you want to test your project locally, you can use the following commands:

```bash
# Starts the replica, running in the background
dfx start --background --clean

# Deploys your canisters to the replica and generates your candid interface
dfx deploy
```

Once the job completes, your application will be available at `http://localhost:4943?canisterId={asset_canister_id}`.
