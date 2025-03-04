use std::collections::HashMap;

const KOTLIN_ROOT_OBJECT_TEMPLATE: &str = r#"package {package_name};

object {name} {
    init {
        System.loadLibrary("{library_name}")
    }

    {contents}
}
"#;
const KOTLIN_OBJECT_TEMPLATE: &str = r#"object {name} {
    {contents}
}
"#;

struct KotlinObject {
    name: String,
    members: Vec<String>,
    child_objects: Vec<KotlinObject>,
}

pub fn get_name_components(binding: &String) -> Vec<String> {
    match binding.split("fun ").nth(1) {
        Some(after_fun) => match after_fun.split("(").nth(0) {
            Some(name) => name.split("_").map(|s| s.to_string()).collect(),
            None => Vec::new(),
        },
        None => Vec::new(),
    }
}

pub fn replace_name(binding: &String, new_name: &String) -> String {
    let components = get_name_components(binding);
    binding.replace(&components.join("_"), new_name)
}

pub fn generate_kotlin_file(
    root_object_name: &str,
    package_name: &str,
    library_name: &str,
    bindings: &Vec<String>,
) -> String {
    let mut root = KotlinObject {
        name: root_object_name.to_string(),
        members: Vec::new(),
        child_objects: Vec::new(),
    };

    // Create nested object hierarchy from bindings
    let mut object_map: HashMap<Vec<String>, Vec<String>> = HashMap::new();
    for binding in bindings {
        let components = get_name_components(binding);
        if components.len() > 1 {
            let key = components[..components.len() - 1].to_vec();
            object_map
                .entry(key)
                .or_insert(Vec::new())
                .push(binding.clone());
        } else {
            root.members.push(binding.clone());
        }
    }

    // Create nested objects from grouped bindings
    for (path, group) in object_map {
        let mut current = &mut root;
        let mut child_index = None;

        // Create/traverse object hierarchy for this path
        for component in &path[..path.len() - 1] {
            if let Some(index) = child_index {
                current = &mut current.child_objects[index];
            }

            child_index = current
                .child_objects
                .iter()
                .position(|o| o.name == *component);

            if child_index.is_none() {
                current.child_objects.push(KotlinObject {
                    name: component.clone(),
                    members: Vec::new(),
                    child_objects: Vec::new(),
                });
                child_index = Some(current.child_objects.len() - 1);
            }
        }

        if let Some(index) = child_index {
            current = &mut current.child_objects[index];
        }

        // Create the leaf object
        let leaf_name = &path[path.len() - 1];
        let mut leaf = KotlinObject {
            name: leaf_name.clone(),
            members: Vec::new(),
            child_objects: Vec::new(),
        };

        // Process each binding in the group
        for binding in group {
            let components = get_name_components(&binding);
            let new_name = components.last().unwrap().to_string();
            let modified_binding = replace_name(&binding, &new_name);
            leaf.members.push(modified_binding);
        }

        current.child_objects.push(leaf);
    }

    // Generate the string representation
    let mut contents = String::new();

    // Add root members
    for member in &root.members {
        contents.push_str(&format!("{}\n", member));
    }

    // Add child objects recursively
    fn add_child_objects(obj: &KotlinObject, contents: &mut String, indent: usize) {
        let mut child_contents = String::new();
        let mut is_first = true;
        for member in &obj.members {
            let indent = "\n    ";
            child_contents.push_str(&format!("{}{}", if is_first { "" } else { indent }, member));
            is_first = false;
        }

        for child in &obj.child_objects {
            add_child_objects(child, &mut child_contents, indent + 1);
        }

        let child_str = KOTLIN_OBJECT_TEMPLATE
            .replace("{name}", &obj.name)
            .replace("{contents}", &child_contents);

        // Replace each new line with the new line followed by the indent string
        let indented_child_str = child_str.replace("\n", "\n    ");

        contents.push_str(&indented_child_str);
    }

    for child in &root.child_objects {
        add_child_objects(child, &mut contents, 0);
    }

    // Return the complete Kotlin file
    KOTLIN_ROOT_OBJECT_TEMPLATE
        .replace("{package_name}", package_name)
        .replace("{name}", &root.name)
        .replace("{library_name}", library_name)
        .replace("{contents}", &contents)
}
