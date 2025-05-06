use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yew_router::prelude::*;
use log::info;
use yew::platform::spawn_local;
use gloo_net::http::Request;
use serde::{Deserialize, Serialize};
use serde_json::json;
use web_sys::HtmlInputElement;

#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Overview,
    #[at("/impressum")]
    Impressum,
    #[at("/404")]
    NotFound,
}


fn switch(routes: Route) -> Html {
    match routes {
        Route::Overview => html! { <OverviewPage /> },
        Route::Impressum => html! { <ImpressumPage /> },
        Route::NotFound => html! { <h1>{ "404 - Not Found" }</h1> },
    }
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}

#[function_component(OverviewPage)]
fn overview() -> Html {
    html!{
        <div class="container font-monospace">
            <div class="col">
                <div class="row my-3">
                    <TopBarComponent/>
                </div>
                <div class="row my-1">
                    <TodoList/>
                </div>
            </div>
        </div>
    } 
}



#[function_component(ImpressumPage)]
fn impressum() -> Html {
    html!{
        <div class="container font-monospace">
            <div class="col">
                <div class="row my-3">
                    <TopBarComponent/>
                </div>
                <div class="row my-1">
                    <h1>{"Impressum"}</h1>
                    <p>{"Yunusemre Kilic. Github: melonsorbet"}</p>
                </div>
            </div>
        </div>
    }
}

#[derive(Serialize, Clone, Deserialize, Default, Properties, PartialEq)]
pub struct TaskProp {
    pub id: i32,
    pub date: String,
    pub inhalt: String,
    pub percent: i32,
}
#[function_component(TaskComponent)]
fn task(props: &TaskProp) -> Html {
    let is_editing = use_state(|| false);
    let edited_task = use_state(|| props.clone());
    
    // Create callbacks
    let on_edit = {
        let is_editing = is_editing.clone();
        Callback::from(move |_| {
            is_editing.set(!*is_editing);
        })
    };

    let on_description_input = {
        let edited_task = edited_task.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut current_task = (*edited_task).clone();
            current_task.inhalt = input.value();
            edited_task.set(current_task);
        })
    };

    let on_date_input = {
        let edited_task = edited_task.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut current_task = (*edited_task).clone();
            current_task.date = input.value();
            edited_task.set(current_task);
        })
    };

    let on_percent_input = {
        let edited_task = edited_task.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let percent = input.value().parse().unwrap_or(0);
            let mut current_task = (*edited_task).clone();
            current_task.percent = percent;
            edited_task.set(current_task);
        })
    };

    let on_update = {
        let is_editing = is_editing.clone();
        let edited_task = edited_task.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let task = (*edited_task).clone();
            let is_editing_clone = is_editing.clone(); // Clone before moving into spawn_local
            
            spawn_local(async move {
                let body = json!({
                    "id": task.id,
                    "date": task.date,
                    "inhalt": task.inhalt,
                    "percent": task.percent
                });
    
                match Request::put("http://localhost:3000/task")
                    .header("Content-Type", "application/json")
                    .body(body.to_string())
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(_) => {
                        log::info!("Task updated successfully");
                        is_editing_clone.set(false); // Use the cloned handle here
                    }
                    Err(e) => log::error!("Failed to update task: {:?}", e),
                }
            });
        })
    };
    
    let on_delete = {
        let task_id = props.id;
        Callback::from(move |_| {
            spawn_local(async move {
                let result = Request::delete("http://localhost:3000/task")
                    .header("Content-Type", "application/json")
                    .body(json!({ "id": task_id }).to_string())
                    .unwrap()
                    .send()
                    .await;

                match result {
                    Ok(_) => log::info!("Deleted task {}", task_id),
                    Err(e) => log::error!("Error deleting task: {:?}", e),
                }
            });
        })
    };

    // Clone for modal close button
    let on_edit_modal_close = on_edit.clone();

    html! {
        <div class="card mb-3">
            <div class="card-body">
                <div class="d-flex justify-content-between align-items-center">
                    <h5 class="card-title">{ &props.inhalt }</h5>
                    <span class="badge bg-primary">{ format!("{}%", props.percent) }</span>
                </div>
                <p class="card-text text-muted">
                    <small>{ "Due: " }{ &props.date }</small>
                </p>
                <div class="progress mb-2">
                    <div 
                        class={classes!(
                            "progress-bar",
                            if props.percent >= 100 { "bg-success" } else { "" }
                        )} 
                        role="progressbar" 
                        style={format!("width: {}%", props.percent)}
                        aria-valuenow={props.percent.to_string()}
                        aria-valuemin="0"
                        aria-valuemax="100"
                    />
                </div>
                <div class="d-flex justify-content-end gap-2">
                    <button class="btn btn-sm btn-outline-secondary" onclick={on_edit}>
                        {"Edit"}
                    </button>
                    <button class="btn btn-sm btn-outline-danger" onclick={on_delete}>
                        {"Delete"}
                    </button>
                </div>
            </div>

            // Edit Modal
            if *is_editing {
                <div class="modal" tabindex="-1" style="display: block;">
                    <div class="modal-dialog">
                        <div class="modal-content">
                            <div class="modal-header">
                                <h5 class="modal-title">{"Edit Task"}</h5>
                                <button type="button" class="btn-close" onclick={&on_edit_modal_close}/>
                            </div>
                            <div class="modal-body">
                                <form onsubmit={on_update}>
                                    <div class="form-group">
                                        <label for="taskDescription">{"Task Description"}</label>
                                        <textarea 
                                            class="form-control" 
                                            id="taskDescription" 
                                            rows=4 
                                            placeholder="Enter task description"
                                            oninput={on_description_input}
                                            value={edited_task.inhalt.clone()}
                                        />
                                    </div>
                                    <div class="form-group">
                                        <label for="datepicker">{"Due Date"}</label>
                                        <input 
                                            type="date" 
                                            class="form-control" 
                                            id="datepicker" 
                                            placeholder="Select date"
                                            oninput={on_date_input}
                                            value={edited_task.date.clone()}
                                        />
                                    </div>
                                    <div class="form-group">
                                        <label for="percentComplete">{"Percent Complete"}</label>
                                        <input 
                                            type="number" 
                                            class="form-control" 
                                            id="percentComplete" 
                                            placeholder="0-100"
                                            min="0"
                                            max="100"
                                            oninput={on_percent_input}
                                            value={edited_task.percent.to_string()}
                                        />
                                    </div>
                                    <div class="modal-footer">
                                        <button type="button" class="btn btn-secondary" onclick={on_edit_modal_close}>
                                            {"Close"}
                                        </button>
                                        <button type="submit" class="btn btn-primary">
                                            {"Save Changes"}
                                        </button>
                                    </div>
                                </form>
                            </div>
                        </div>
                    </div>
                </div>
                <div class="modal-backdrop fade show"></div>
            }
        </div>
    }
}

#[function_component(TodoList)]
fn todo() -> Html {
    let tasks = use_state(|| vec![]);

    {
        let tasks = tasks.clone();
        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched = Request::get("http://localhost:3000/task")
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<TaskProp>>()
                    .await
                    .unwrap();
                tasks.set(fetched);
            });
            || ()
        });
    }

    html! {
        <div class="container">
            <h2>{ "All Tasks" }</h2>
            { for tasks.iter().map(|task| html! {
                <TaskComponent
                    id={task.id} 
                    date={task.date.clone()} 
                    inhalt={task.inhalt.clone()} 
                    percent={task.percent} 
                />
            })}
        </div>
    }
}

#[function_component(TopBarComponent)]
fn top_bar() -> Html {
    html!{
        <div>
            <div class="row text-center ">
                <Link<Route> classes="btn btn-outline-primary col mx-1" to={Route::Overview}>
                    {"Overview"}
                </Link<Route>>
                <button class="btn btn-outline-primary col mx-1" data-bs-toggle="modal" data-bs-target="#exampleModal">
                    {"Create new Todo"}
                </button>
                <Link<Route> classes="btn btn-outline-primary col mx-1" to={Route::Impressum}>
                    {"Impressum"}
                </Link<Route>>
            </div>
            <AddTaskModal/>
        </div>  
    }   
}

#[function_component(AddTaskModal)]
fn add_task_pop_up() -> Html {
    let task = use_state(|| TaskProp::default());
    
    let on_description_input = {
        let task = task.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut current_task = (*task).clone();
            current_task.inhalt = input.value();
            task.set(current_task);
        })
    };
    
    let on_date_input = {
        let task = task.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut current_task = (*task).clone();
            current_task.date = input.value();
            task.set(current_task);
        })
    };
    
    let on_percent_input = {
        let task = task.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let percent = input.value().parse().unwrap_or(0);
            let mut current_task = (*task).clone();
            current_task.percent = percent;
            task.set(current_task);
        })
    };
    
    let task_state = task.clone();
    let onsubmit = {
        let task_state = task.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let task_state = task_state.clone();
            let task = (*task_state).clone();
            
            spawn_local(async move {
                let body = json!({
                    "id": task.id,
                    "date": task.date,
                    "inhalt": task.inhalt,
                    "percent": task.percent
                });
    
                match Request::post("http://localhost:3000/task")
                    .header("Content-Type", "application/json")
                    .body(body.to_string())
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(_) => {
                        log::info!("Task created successfully");
                        task_state.set(TaskProp::default());
                    }
                    Err(e) => {
                        log::error!("Failed to create task: {:?}", e);
                    }
                }
            });
        })
    };
    
    html!{
        <div class="modal" tabindex="-1" id="exampleModal">
            <div class="modal-dialog">
              <div class="modal-content">
                <div class="modal-header">
                  <h5 class="modal-title">{"Add New Task"}</h5>
                  <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                </div>
                <div class="modal-body">
                    <form onsubmit={onsubmit}>
                        <div class="form-group">
                            <label for="taskDescription">{"Task Description"}</label>
                            <textarea 
                                class="form-control" 
                                id="taskDescription" 
                                rows=4 
                                placeholder="Enter task description"
                                oninput={on_description_input}
                                value={task.inhalt.clone()}
                            />
                        </div>
                        <div class="form-group">
                            <label for="datepicker">{"Due Date"}</label>
                            <input 
                                type="date" 
                                class="form-control" 
                                id="datepicker" 
                                placeholder="Select date"
                                oninput={on_date_input}
                                value={task.date.clone()}
                            />
                        </div>
                        <div class="form-group">
                            <label for="percentComplete">{"Percent Complete"}</label>
                            <input 
                                type="number" 
                                class="form-control" 
                                id="percentComplete" 
                                placeholder="0-100"
                                min="0"
                                max="100"
                                oninput={on_percent_input}
                                value={task.percent.to_string()}
                            />
                        </div>
                        <button type="submit" class="btn btn-primary">{"Submit"}</button>
                    </form>
                </div>
                <div class="modal-footer">
                  <button type="button" class="btn btn-secondary" data-bs-dismiss="modal">{"Close"}</button>
                </div>
              </div>
            </div>
          </div>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());
    yew::Renderer::<App>::new().render();
}