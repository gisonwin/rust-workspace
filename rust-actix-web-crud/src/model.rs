use chrono::prelude::*;
use serde::{Deserialize,Serialize};
use std::sync::{Arc,Mutex};
/**
 *  @Description
 *  @author <a href="mailto:gisonwin@qq.com">GiSon.Win</a>
 *  @Date 2023/11/9 15:49
 */
#[allow(non_snake_case)]
#[derive(Debug,Deserialize,Serialize,Clone)]
pub struct Todo{
    pub id: Option<String>,
    pub title:String,
    pub content:String,
    pub completed:Option<bool>,
    pub createAt:Option<DateTime<Utc>>,
    pub updateAt:Option<DateTime<Utc>>,
}
pub struct AppState{
    pub todo_db:Arc<Mutex<Vec<Todo>>>,
}

impl AppState {
    pub fn init() -> AppState{
        AppState{
            todo_db:Arc::new(Mutex::new(Vec::new())),
        }
    }
}

#[derive!(Debug,Deserialize)]
pub struct QueryOptions{
    pub page:Option<usize>,
    pub limit:Option<usize>,
}

#[allow(non_snake_case)]
#[derive(Debug,Deserialize)]
pub struct UpdateTodoSchema{
    pub title:Option<String>,
    pub content:Option<String>,
    pub completed:Option<bool>,
}