use std::collections::HashMap;
use std::rc::Rc;
use uuid::Uuid;

struct EntityRegistry {
    //(entityUUID, PlanningVariableName, branch)-> planningValue
    value_map: HashMap<Uuid, HashMap<String, HashMap<Uuid, Rc<PlanningValue>>>>,
    //child Uuid, parent Uuid
    relation_map: HashMap<Uuid, Uuid>
}
impl EntityRegistry {
    fn get_value(self, branch: &Uuid, entity_uuid: &Uuid, variable_name: String) -> Option<Rc<PlanningValue>> {
        let entity_map = self.value_map.get(entity_uuid);
        if (entity_map.is_none()) {
            return None
        }
        let entity_map = entity_map.unwrap();
        let temp_result_map = entity_map.get(&variable_name);
        if temp_result_map.is_none() {
            return None;
        }
        let result_map = temp_result_map.unwrap();
        let mut lookup_branch: &Uuid = branch;
        loop {
            let result = result_map.get(lookup_branch);
            if result.is_some() {
                return Some(result.unwrap().clone())
            }//the value wasn't found in the current branch, check if it is in another branch
            let parent_option = self.relation_map.get(lookup_branch);
            if parent_option.is_none() {
                return None
            }
            lookup_branch = parent_option.unwrap();
        }
        panic!("Broke out of the get value loop without returning a value!")
    }
}

enum PlanningValue {
    String(String),
    Int(i32),
    Double(f64),
    Id(Uuid, String) //references other entity through the lookup system (entityUUID, variable)
}