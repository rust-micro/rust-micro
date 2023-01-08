pub async fn get_ip_addr(service_name: &str) -> String {
    let mut client = etcd_client::Client::connect(["localhost:2379"], None)
        .await
        .expect("Failed to connect to etcd");

    let port: u16 = client
        .get(service_name.to_string(), None)
        .await
        .expect("Failed to get service")
        .kvs()
        .first()
        .expect("No service found")
        .value_str()
        .expect("Failed to parse service port")
        .parse()
        .expect("Failed to parse service port");

    let server_addr = format!("http://localhost:{}", port);
    server_addr
}
