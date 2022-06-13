use std::collections::HashMap;
use uuid::Uuid;

struct EntityRegistry {
    map: HashMap<Uuid, HashMap<String, PlanningValue>> //entityUUID, PlanningVariableName, planning value
}

enum PlanningValue {
    String(String),
    Int(i32),
    Double(f64),
    Id(Uuid, String) //references other entity through the lookup system (entityUUID, String)
}

#[cfg(test)]
mod tests {

    #[test]
    fn it_works() {

    }
}
