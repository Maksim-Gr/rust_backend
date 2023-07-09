use crate::error_handlers::CustomError;
use serde::{Serialize, Deserialize};

#[derive(Serializable, Deserialize, AsChangeSet, Intsertable)]
pub struct Employee {
    pub first_name: String,
    pub last_name: String,
    pub department: String,
    pub salary: i32,
    pub age: i32,
}

impl Employee {
    fn from(employee: Employee) -> Employee {
        Employee {
            first_name: employee.first_name,
            last_name: employee.last_name,
            department: employee.department,
            salary: employee.salary,
            age: employee.age
        }
    }
}