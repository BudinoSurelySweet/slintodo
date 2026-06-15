use slint::{Model, ModelRc, SharedString, VecModel};
use std::rc::Rc;

slint::include_modules!();

fn main() -> Result<(), slint::PlatformError> {
    let app = App::new()?;

    let todos = Rc::new(VecModel::<(i32, SharedString)>::default());
    let completed_todos = Rc::new(VecModel::<(i32, SharedString)>::default());

    connect_backend_data_to_frontend(&app, todos.clone(), completed_todos.clone());
    setup_callback(&app, todos.clone(), completed_todos.clone());

    app.run()?;

    Ok(())
}

fn connect_backend_data_to_frontend(
    app: &App,
    todos: Rc<VecModel<(i32, SharedString)>>,
    completed_todos: Rc<VecModel<(i32, SharedString)>>,
) {
    app.set_todo_info(ModelRc::from(todos));
    app.set_completed_todo_info(ModelRc::from(completed_todos));
}

fn setup_callback(
    app: &App,
    todos: Rc<VecModel<(i32, SharedString)>>,
    completed_todos: Rc<VecModel<(i32, SharedString)>>,
) {
    // Callback logic: insert todo
    let t = todos.clone();
    app.on_insert_todo(move |text| {
        if text.is_empty() {
            return;
        }

        let id = t.row_count() as i32;
        t.push((id, text));
    });

    // Callback logic: complete todo
    let t = todos.clone();
    let c = completed_todos.clone();
    app.on_complete_todo(move |id, text| {
        if let Some(index) = t.iter().position(|x| x.0 == id) {
            c.push((id, text));
            t.remove(index);
        }
    });

    // Callback logic: complete todo
    let t = todos.clone();
    let c = completed_todos.clone();
    app.on_restore_todo(move |id, text| {
        if let Some(index) = c.iter().position(|x| x.0 == id) {
            t.push((id, text));
            c.remove(index);
        }
    });

    // Callback logic: delete all completed todos
    let c = completed_todos.clone();
    app.on_delete_all_completed(move || {
        c.clear();
    });

    // Callback logic: restore all completed todos
    let t = todos.clone();
    let c = completed_todos.clone();
    app.on_restore_all_completed(move || {
        for todo in c.iter() {
            t.push(todo);
        }

        c.clear();
    });
}
