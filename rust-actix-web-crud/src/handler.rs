use std::fmt::Debug;
/**
 *  @Description
 *  @author <a href="mailto:gisonwin@qq.com">GiSon.Win</a>
 *  @Date 2023/11/9 16:00
 */
use crate::{
    model::{AppState, QueryOptions, Todo, UpdateTodoSchema},
    response::{GenericResponse, SingleTodoResponse, TodoData, TodoListResponse},
};
use actix_web::{delete, get, patch, post, web, HttpResponse, Responder};
use chrono::prelude::*;
use uuid::Uuid;

#[get("/api/healthChecker")]
async fn health_checker_handler() -> impl Responder {
    const MESSAGE: &str = "Build Simple CRUD API with Rust and Actix Web";

    let response_json = &GenericResponse {
        status: "success".to_string(),
        message: MESSAGE.to_string()
    };
    HttpResponse::Ok().json(response_json)
}

#[get("/todos")]
async fn todos_list_handler(opts: web::Query<QueryOptions>, data: web::Data<AppState>) -> impl Responder {
    let todos = data.todo_db.lock().unwrap();
    let limit = opts.limit.unwrap_or(10);
    let offset = (opts.page.unwrap_or(1) - 1) * limit;

    let todos_vec: Vec<Todo> = todos.clone().into_iter().skip(offset).take(limit).collect();

    let json_response = TodoListResponse {
        status: "success".to_string(),
        results: todos.len(),
        todos_vec,
    };
    HttpResponse::Ok().json(json_response)
}


//create a record
#[post("/todos")]
async fn create_todo_handler(mut body: web::Json<Todo>, data: web::Data<AppState>) -> impl Responder {
    let mut vec = data.todo_db.lock().unwrap();
    let todo = vec.iter().find(|todo| todo.title == body.title);

    if todo.is_some() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Todo with title :'{}' already exists", body.title),
        };
        return HttpResponse::Conflict().json(error_response);
    }

    let uuid_id = Uuid::new_v4();
    format!("{uuid_id}");
    let datetime = Utc::now();

    body.id = Some(uuid_id.to_string());
    body.completed = Some(false);
    body.createAt = Some(datetime);
    body.updateAt = Some(datetime);

    let todo = body.to_owned();

    vec.push(body.into_inner());

    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo },
    };
    HttpResponse::Ok().json(json_response)
}

//query a single record
#[get("/todos/{id}")]
async fn get_todo_handler(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let vec = data.todo_db.lock().unwrap();
    let id = path.into_inner();
    let todo = vec.iter().find(|todo| todo.id == Some(id.to_owned()));
    if todo.is_none() {
        let error_response = GenericResponse {
            status: "fail".to_string(),
            message: format!("Todo with id: {} not found", id),
        };
        return HttpResponse::NotFound().json(error_response);
    }

    let todo = todo.unwrap();
    let json_response = SingleTodoResponse {
        status: "success".to_string(),
        data: TodoData { todo_data: todo.clone() },
    };
    HttpResponse::Ok().json(json_response)
}

//edit a record
#[patch("/todos/{id}")]
async fn edit_todo_handler(path: web::Path<String>, body: web::Json<UpdateTodoSchema>, data: web::Data<AppState>) -> impl Responder {
    let mut vec = data.todo_db.lock().unwrap();

    let id = path.into_inner();

    let todo = vec.iter_mut().find(|todo| todo.id == Some(id.to_owned()));

    if !(todo.is_none()) {
        let todo = todo.unwrap();
        let datetime = Utc::now();
        let title
            = body.title.to_owned().unwrap_or(todo.title.to_owned());
        let content = body.content.to_owned().unwrap_or(todo.content.to_owned());

        let payload = Todo {
            id: todo.id.to_owned(),
            title: if !title.is_empty() {
                title
            } else {
                todo.title.to_owned()
            },
            content: if !content.is_empty() {
                content
            } else {
                todo.content.to_owned()
            },
            completed: if body.completed.is_some() {
                body.completed
            } else {
                todo.completed
            },
            createAt: todo.createAt,
            updateAt: Some(datetime),
        };

        *todo = payload;

        let response = SingleTodoResponse {
            status: "success".to_string(),
            data: TodoData { todo: todo.clone() },
        };
        return HttpResponse::Ok().json(response);
    }

    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Todo with id: {} not found", id),
    };
    HttpResponse::NotFound().json(error_response)
}

//delete a record
#[delete("/todos/{id}")]
async fn delete_todo_handler(path: web::Path<String>, data: web::Data<AppState>) -> impl Responder {
    let mut vec = data.todo_db.lock().unwrap();
    let id
        = path.into_inner();
    let todo = vec.iter_mut().find(|todo| todo.id == Some(id.to_owned()));
    if !(todo.is_none()) {
        vec.retain(|todo| todo.id != Some(id.to_owned()));
        return HttpResponse::NoContent().finish();
    }
    let error_response = GenericResponse {
        status: "fail".to_string(),
        message: format!("Todo with id: {} not found", id),
    };
    HttpResponse::NotFound().json(error_response)
}

//config

pub fn config(conf: &mut web::ServiceConfig) {
    let scope = web::scope("/api")
        .service(health_checker_handler)
        .service(todos_list_handler)
        .service(create_todo_handler)
        .service(get_todo_handler)
        .service(edit_todo_handler)
        .service(delete_todo_handler);

    conf.service(scope);
}