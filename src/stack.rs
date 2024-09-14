//use openstack::Cloud;
use openstack_reqwest::{Method, Url};
use osauth::{client::AuthenticatedClient, EndpointFilters};

pub enum Compute {
    ListServers,
    ListHypervisors,
    ServerID(String),
    HypervisorID(String),
}

pub async fn request(client: AuthenticatedClient, call: Compute) -> reqwest::Response {
    let filters = &EndpointFilters::default();
    let endpoint = client.get_endpoint("compute", filters).await.unwrap();
    let end = match call {
        Compute::ListServers => "servers".to_string(),
        Compute::ServerID(o) => format!("servers/{}", o),
        Compute::ListHypervisors => "os-hypervisors".to_string(),
        Compute::HypervisorID(o) => format!("os-hypervisors/{o}"),
    };
    let rb = client.request(
        Method::GET,
        Url::parse(format!("{endpoint}/{end}").as_str()).unwrap(),
    );
    let res = rb.send().await.unwrap();
    res
}

pub async fn list_servers(client: AuthenticatedClient) -> reqwest::Response {
    let filters = &EndpointFilters::default();
    let endpoint = client.get_endpoint("compute", filters).await.unwrap();
    let rb = client.request(
        Method::GET,
        Url::parse(format!("{}/servers", endpoint.to_string()).as_str()).unwrap(),
    );
    let res = rb.send().await.unwrap();
    res
}

pub async fn list_hypervisors(client: AuthenticatedClient) -> reqwest::Response {
    let filters = &EndpointFilters::default();
    let endpoint = client.get_endpoint("compute", filters).await.unwrap();
    let rb = client.request(
        Method::GET,
        Url::parse(format!("{}/os-hypervisors", endpoint.to_string()).as_str()).unwrap(),
    );
    let res = rb.send().await.unwrap();
    res
}

pub async fn get_hypervisor(client: AuthenticatedClient) -> reqwest::Response {
    let filters = &EndpointFilters::default();
    let endpoint = client.get_endpoint("compute", filters).await.unwrap();
    let rb = client.request(
        Method::GET, 
        Url::parse(format!("{}/os-hypervisors/1", endpoint.to_string()).as_str()).unwrap(),
    );
    let res = rb.send().await.unwrap();
    res
}
