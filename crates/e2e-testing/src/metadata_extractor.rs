use regex::Regex;

pub struct AppRoute {
    pub name: String,
    pub route_url: String,
    pub wildcard: bool,
}

pub struct Metadata {
    pub name: String,
    pub base: String,
    pub app_routes: Vec<AppRoute>,
    pub version: String,
}

impl Drop for Metadata {
    fn drop(&mut self) {
        print!("dropping app for {}", self.name)
    }
}

pub fn extract_version_from_logs(appname: &str, logs: &str) -> String {
    let re: Regex = Regex::new(format!("Uploading {} version (.*)...", appname).as_str()).unwrap();
    return re.find(logs).unwrap().as_str().to_string();
}

pub fn extract_routes_from_logs(appname: &str, logs: &str) -> String {
    let re: Regex = Regex::new(format!("Uploading {} version (.*)...", appname).as_str()).unwrap();
    return re.find(logs).unwrap().as_str().to_string();
}

pub fn extract_app_metadata_from_logs(appname: &str, logs: &str) -> Metadata {
    let version = extract_version_from_logs(appname, logs);
    return Metadata {
        name: appname.to_string(),
        base: "".to_string(),
        version: version,
        app_routes: vec![],
    };
}
