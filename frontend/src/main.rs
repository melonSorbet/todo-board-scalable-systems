use yew::prelude::*;
use wasm_bindgen::prelude::*;
use yew_router::prelude::*;
use log::info;
use yew::platform::spawn_local;
use gloo_net::http::Request;
#[derive(Clone, Routable, PartialEq)]
enum Route {
    #[at("/")]
    Overview,
    #[at("/impressum")]
    Impressum
    ,
    #[at("/404")]
    NotFound,
}
fn switch(routes: Route) -> Html {
    match routes {
        Route::Overview => html! { <Overview /> },
        Route::Impressum => html! { <Impressum /> },
        Route::NotFound => html! { <h1>{ "404 - Not Found" }</h1> },
    }
}

#[wasm_bindgen(inline_js = "
    export function initDatePicker() {
        flatpickr('#datepicker', {
            enableTime: false,
            dateFormat: 'Y-m-d',
        });
    }
")]
unsafe extern "C" {
    unsafe fn initDatePicker();
}

#[function_component(App)]
fn app() -> Html {
    html! {
        <BrowserRouter>
            <Switch<Route> render={switch} />
        </BrowserRouter>
    }
}
async fn get_tasks(){
        
        
}
#[function_component(Overview)]
fn overview() -> Html {
    
    html!{
        <div class="container font-monospace">
            <div class="col">
                <div class="row my-3">
                    <TopBar/>
                </div>
                <div class="row my-1">
                    <Todo/>
                </div>
            </div>
        </div>
    } 
}
#[function_component(Impressum)]
fn impressum() -> Html{
    html!{
        <div >{"frontend"}</div>
    }
}
use serde::{Deserialize, Serialize};
#[derive(Serialize, Clone,Deserialize,Default,Properties,PartialEq)]
pub struct TaskProp {
    pub id: i32,
    pub date: String,
    pub inhalt: String,
    pub percent: i32,
}
#[function_component]
fn Task(props: &TaskProp) -> Html {
    html! {
        <div class="card mb-3">
            <div class="card-body">
                <div class="d-flex justify-content-between align-items-center">
                    <h5 class="card-title">{ &props.inhalt}</h5>
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
                    >
                    </div>
                </div>
                <div class="d-flex justify-content-end gap-2">
                    <button class="btn btn-sm btn-outline-secondary">{"Edit"}</button>
                    <button class="btn btn-sm btn-outline-danger">{"Delete"}</button>
                </div>
            </div>
        </div>
    }
}

#[function_component]
fn Todo() -> Html{
    
    let tasks = use_state(|| vec![]);

    {
        let tasks = tasks.clone();
        use_effect(move || {
            wasm_bindgen_futures::spawn_local(async move {
                let fetched = Request::get("http://localhost:3000/task")
                    .send()
                    .await
                    .unwrap()
                    .json::<Vec<TaskProp>>() // needs Deserialize
                    .await
                    .unwrap();

                tasks.set(fetched);
            });
            || ()
        });
    }


    html!{

        html! {
        <div class="container">
            <h2>{ "All Tasks" }</h2>
            { for tasks.iter().map(|task| html! {
                <Task
                    id={task.id} 
                    date={task.date.clone()} 
                    inhalt={task.inhalt.clone()} 
                    percent={task.percent} 
                />
            })}
        </div>
    }
    }
}
#[function_component]
fn TopBar() -> Html{
    let create_new_todo = Callback::from(move |_| {
        //request to backend
        
    });
    html!{
        <div>
            <div class="row text-center ">
                <button class="btn btn-outline-primary col mx-1">{"Overview"}</button>
                <button class="btn btn-outline-primary col mx-1" data-bs-toggle="modal" data-bs-target="#exampleModal" onclick={create_new_todo.clone()}>{"Create new Todo"}</button>
                <button class="btn btn-outline-primary col mx-1">{"Impressum"}</button>
            </div>
            <AddTaskPopUp/>
        </div>  

    }   
}
use yew::prelude::*;
use web_sys::HtmlInputElement;
use serde_json::json;
use wasm_bindgen::JsValue;

#[function_component]
fn AddTaskPopUp() -> Html {
    let task = use_state(|| TaskProp {
        id: 0,
        date: String::new(),
        inhalt: String::new(),
        percent: 0,
    });
    
    use_effect(|| {
        unsafe {
            initDatePicker();
        }
        || ()
    });
    
    let on_description_input = {
        let task = task.clone();
        Callback::from(move |e: InputEvent| {
            let input: HtmlInputElement = e.target_unchecked_into();
            let mut current_task = (*task).clone();
            current_task.inhalt= input.value();
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
    
    let onsubmit = {
        let task = task.clone();
        Callback::from(move |e: SubmitEvent| {
            e.prevent_default();
            let task = (*task).clone();
            
            spawn_local(async move {
                let body = json!({
                    "id": task.id,
                    "date": task.date,
                    "inhalt": task.inhalt,
                    "percent": task.percent
                });

                match Request::post("http://localhost:3000/task")
                    .header("Content-Type", "application/json")
                    .body(JsValue::from_str(&body.to_string()))
                    .unwrap()
                    .send()
                    .await
                {
                    Ok(response) => {
                        log::info!("Task created successfully");
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
                                type="text" 
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
