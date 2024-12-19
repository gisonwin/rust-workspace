use serde::Serialize;
use crate::model::Todo;

/**
 *  @Description
 *  @author <a href="mailto:gisonwin@qq.com">GiSon.Win</a>
 *  @Date 2023/11/9 15:49
 */
#[derive(Serialize)]
pub struct GenericResponse {
    pub status: String,
    pub message: String,
}

#[derive(Debug, Serialize)]
pub struct TodoData {
    pub todo: Todo,
}

#[derive(Debug, Serialize)]
pub struct SingleTodoResponse {
    pub status: String,
    pub data: TodoData,
}

#[derive(Debug, Serialize)]
pub struct TodoListResponse {
    pub status: String,
    pub results: usize,
    pub todos: Vec<Todo>,
}