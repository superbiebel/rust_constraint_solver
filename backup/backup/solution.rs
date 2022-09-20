use std::collections::HashMap;
use std::ops::Deref;
use std::rc::Rc;
use uuid::Uuid;

struct Solution {
    entity_map: HashMap<EntityVariable, Rc<PlanningValue>>
}
trait Entity<A,B,C,D,E> {
    fn amount_implemented() -> i8 {
        5
    }
    fn get_A() -> A;
    fn get_B() -> B;
    fn get_C() -> C;
    fn get_D() -> D;
    fn get_E() -> E;

}
impl Solution {
    fn get_value(self, entity_variable: &EntityVariable) -> Option<Rc<PlanningValue>> {
        let value_opt = self.entity_map.get(entity_variable);
        if value_opt.is_none() {
            return None;
        }
        Some(value_opt.unwrap().clone())
    }
    fn set_value(mut self, entity_variable: EntityVariable, value: PlanningValue) {
        self.entity_map.insert(entity_variable, Rc::new(value));
    }
}
impl Clone for Solution {
    fn clone(&self) -> Self {
        let mut map: HashMap<EntityVariable, Rc<PlanningValue>> = HashMap::new();
        self.entity_map.iter().for_each(|entry| {
            let val = entry.1.deref().clone();
            map.insert(entry.0.clone(), Rc::new(val));
        });
        Solution {
            entity_map: map
        }
    }
}
#[derive(PartialEq, Hash, Eq)]
struct EntityVariable {
    entity_uuid: Uuid,
    variable_name: String,
}
impl Clone for EntityVariable {
    fn clone(&self) -> Self {
        EntityVariable {
            entity_uuid: self.entity_uuid.clone(),
            variable_name: self.variable_name.clone(),
        }
    }
}
#[derive(Clone)]
pub enum PlanningValue {
    String(String),
    Int8(i8),
    Int16(i16),
    Int32(i32),
    Int64(i64),
    Int128(i128),
    Decimal32,
    Decimal64(f64),
    EntityVariable(Uuid, String), //references other entity through the lookup system (entityUUID, variable)
    FactVariable(Uuid, String)
}