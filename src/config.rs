use serde::Deserialize;
use serde_json::from_str;

#[derive(Deserialize)]
pub struct Input {
    #[serde(rename = "type")]
    pub rust_type: String,
    pub name: String,
}

#[derive(Deserialize)]
pub struct Member {
    #[serde(rename = "type")]
    pub member_type: String,
    pub name: String,
    pub inputs: Option<Vec<Input>>,
    pub output: Option<String>,
}

#[derive(Deserialize)]
pub struct Crate {
    pub name: String,
    pub members: Vec<Member>,
}

pub fn parse(json: &str) -> Result<Vec<Crate>, serde_json::Error> {
    from_str(json)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_empty_array() {
        let json = "[]";
        let result = parse(json).unwrap();
        assert_eq!(result.len(), 0);
    }

    #[test]
    fn test_parse_simple_crate() {
        let json = r#"[
            {
                "name": "test_crate",
                "members": [
                    {
                        "type": "function",
                        "name": "test_fn",
                        "inputs": null,
                        "output": null
                    }
                ]
            }
        ]"#;
        let result = parse(json).unwrap();
        assert_eq!(result.len(), 1);
        assert_eq!(result[0].name, "test_crate");
        assert_eq!(result[0].members.len(), 1);
        assert_eq!(result[0].members[0].member_type, "function");
        assert_eq!(result[0].members[0].name, "test_fn");
    }

    #[test]
    fn test_parse_with_inputs() {
        let json = r#"[
            {
                "name": "test_crate",
                "members": [
                    {
                        "type": "function",
                        "name": "test_fn",
                        "inputs": [
                            {
                                "type": "String",
                                "name": "input1"
                            }
                        ],
                        "output": "bool"
                    }
                ]
            }
        ]"#;
        let result = parse(json).unwrap();
        let member = &result[0].members[0];
        assert_eq!(member.inputs.as_ref().unwrap().len(), 1);
        assert_eq!(member.inputs.as_ref().unwrap()[0].rust_type, "String");
        assert_eq!(member.inputs.as_ref().unwrap()[0].name, "input1");
        assert_eq!(member.output.as_ref().unwrap(), "bool");
    }

    #[test]
    fn test_parse_invalid_json() {
        let json = "invalid json";
        assert!(parse(json).is_err());
    }
}
