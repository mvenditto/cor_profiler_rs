use serde_yaml;
use serde_yaml::Value;

pub(crate) struct TargetMethod {
    assembly_name: String,
    type_name: String,
    method_name: String,
    type_names: Vec<String>
}

pub(crate) struct HookMethod {
    assembly_name: String,
    assembly_version: String,
    assembly_public_key_token: Vec<u8>,
    assembly_culture: String,
    type_name: String,
    method_name: String,
    type_names: Vec<String>
}

pub(crate) struct HookEntry {
    name: String,
    target: TargetMethod,
    hook: HookMethod
}

pub(crate) struct HooksRegistry {
    hooks: Vec<HookEntry>
}

impl HooksRegistry {
    pub fn load(cfg_file: &str) -> Self {
        HooksRegistry {
            hooks: load_hooks(cfg_file).unwrap()
        }
    }

    // TODO: consider assembly also
    fn get_function_hooks(self, fully_qualified_function_name: &str) -> Option<HookMethod> {
        for x in self.hooks {
            let target_method_name = format!("{}{}",
                x.target.type_name, x.target.method_name
            );
            if target_method_name == fully_qualified_function_name {
                return Some(x.hook);
            }
        }
        None
    }
}

fn parse_hook_entry(vals: &Vec<Value>) -> Vec<HookEntry> {
    let mut hooks = Vec::with_capacity(vals.len());
    for v in vals {
        let name = v["name"].as_str().unwrap().to_string();
        let target = parse_target_method(&v["target"]);
        let hook = parse_hook_method(&v["hook"]);
        hooks.push(HookEntry {
            name,
            target,
            hook
        });
    }
    return hooks;
}

fn parse_hook_method(v: &Value) -> HookMethod {
    let assembly = &v["assembly"];
    let assembly_name = assembly["name"].as_str().unwrap();
    let assembly_version = assembly["version"].as_str().unwrap();
    let assembly_culture = assembly["culture"].as_str().unwrap();
    let assembly_key = assembly["public_key_token"].as_sequence().unwrap();
    let mut pub_key_token: Vec<u8> = Vec::with_capacity(assembly_key.len());
    for b in assembly_key {
        let byte = b.as_u64().unwrap() as u8;
        pub_key_token.push(byte);
    }
    let type_name = v["type"].as_str().unwrap();
    let method = &v["method"];
    let method_name = method["name"].as_str().unwrap();
    let ret_type_name = method["return_type"].as_str().unwrap();
    let arg_types = method["argument_types"].as_sequence().unwrap();
    let mut type_names: Vec<String> = Vec::with_capacity(arg_types.len() + 1);
    type_names.push(ret_type_name.to_string());
    for t in arg_types {
        let name = t.as_str().unwrap();
        type_names.push(name.to_string());
    }
    return HookMethod {
        assembly_name: assembly_name.to_string(),
        assembly_version: assembly_version.to_string(),
        assembly_culture: assembly_culture.to_string(),
        assembly_public_key_token: pub_key_token,
        type_name: type_name.to_string(),
        method_name: method_name.to_string(),
        type_names
    };
}

fn parse_target_method(v: &Value) -> TargetMethod {
    let assembly_name = v["assembly"].as_str().unwrap();
    let type_name = v["type"].as_str().unwrap();
    let method = &v["method"];
    let method_name = method["name"].as_str().unwrap();
    let ret_type_name = method["return_type"].as_str().unwrap();
    let arg_types = method["argument_types"].as_sequence().unwrap();
    let mut type_names: Vec<String> = Vec::with_capacity(arg_types.len() + 1);
    type_names.push(ret_type_name.to_string());
    for t in arg_types {
        let name = t.as_str().unwrap();
        type_names.push(name.to_string());
    }
    return TargetMethod {
        assembly_name: assembly_name.to_string(),
        type_name: type_name.to_string(),
        method_name: method_name.to_string(),
        type_names
    };
}

fn load_hooks(hooks_cfg_file_yml: &str) -> Result<Vec<HookEntry>, Box<dyn std::error::Error>> {
    let cfg_file = std::fs::File::open(hooks_cfg_file_yml)?;
    let val: Value = serde_yaml::from_reader(cfg_file)?;
    let hooks = val.as_sequence().unwrap();
    return Ok(parse_hook_entry(hooks))
}