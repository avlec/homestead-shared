pub mod models {
    use serde::*;

    #[derive(Clone, Copy, Debug, Serialize, Deserialize)]
    #[serde(rename_all = "lowercase")]
    pub enum Value {
        Signed(i32),
        Unsigned(u32),
        Real(f32),
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Resource {
        pub what: String,
        pub value: Value,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct ContextResource {
        pub context: String,
        pub resource: Resource,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct HardwareAddress(Vec<u8>);

    impl std::fmt::Display for HardwareAddress {
        fn fmt(&self, f: &mut __private::Formatter<'_>) -> std::fmt::Result {
            self.0
                .iter()
                .map(|&v| v.to_string())
                .fold(String::new(), |a, b| {
                    if a.len() == 0 {
                        a + &b
                    } else {
                        a + ":" + &b
                    }
                })
                .fmt(f)
        }
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Capabilities {
        pub produces: Vec<String>, // list of names
        pub consumes: Vec<String>, // list of names
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct Device {
        pub hwaddr: HardwareAddress,
        pub capabilities: Capabilities,
    }

    #[derive(Debug, Serialize, Deserialize)]
    pub struct DevicePut {
        pub hwaddr: HardwareAddress,
        pub resources: Vec<Resource>,
    }
}
