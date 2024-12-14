use std::collections::HashMap;

fn main() {
    
    let mut aps_levels: HashMap<&str, HashMap<&str, &str>> = HashMap::new();

    
    aps_levels.insert("Office Administrator", {
        let mut m = HashMap::new();
        m.insert("Intern", "APS 1-2");
        m.insert("Administrator", "APS 3-5");
        m.insert("Senior Administrator", "APS 5-8");
        m.insert("Office Manager", "EL1 8-10");
        m.insert("Director", "EL2 10-13");
        m.insert("CEO", "SES");
        m
    });

    aps_levels.insert("Lawyer", {
        let mut m = HashMap::new();
        m.insert("Paralegal", "APS 1-2");
        m.insert("Junior Associate", "APS 3-5");
        m.insert("Associate", "APS 5-8");
        m.insert("Senior Associate 1-2", "EL1 8-10");
        m.insert("Senior Associate 3-4", "EL2 10-13");
        m.insert("Partner", "SES");
        m
    });

    
    let role = "Lawyer"; 
    let position = "Associate"; 

    
    if let Some(role_map) = aps_levels.get(role) {
        if let Some(&aps_level) = role_map.get(position) {
            println!("The APS level for a {} with position '{}' is: {}", role, position, aps_level);
        } else {
            println!("Position not found for the role: {}", role);
        }
    } else {
        println!("Role not found.");
    }
}
