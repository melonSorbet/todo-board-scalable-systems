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
    use_effect(|| {
        wasm_bindgen_futures::spawn_local(async {
            let response = Request::get("https://api.github.com/users/rust-lang")
                .send()
                .await
                .unwrap()
                .text()
                .await
                .unwrap();
            
            web_sys::console::log_1(&response.into());
        });
        
        || () // No cleanup needed
    });
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
#[function_component]
fn Todo() -> Html{
    html!{
        <div class="row">
            <div class="col">

                <UnfinishedTasks/>
            </div>
            <div class="col">

                <FinishedTasks/>
            </div>
        </div>
    }
}
#[function_component]
fn FinishedTasks() -> Html{
    html!{
        <div id={"finished"} class="text-center border rounded p-3 shadow-sm">
            <div class="fw-bold">
                <h2>{"FINISHED TASKS"}</h2>
            </div>
            <div class="border rounded p-3 d-flex justify-content-center align-items-center my-2" style="height: 100px;">
                <p>{"finished Task 1"}</p>
            </div>
            <div class="border rounded p-3 d-flex justify-content-center align-items-center my-2" style="height: 100px;">
                <p>{"finished Task 2"}</p>
            </div>
        </div>
    }
}
#[function_component]
fn UnfinishedTasks() -> Html {
    html! {
        <div id={"unfinished"} class="text-center border rounded p-3 shadow-sm">
            <div class="fw-bold">
                <h2>{"UNFINISHED TASKS"}</h2>
            </div>

            <div class="border rounded p-3 d-flex justify-content-center align-items-center my-2" style="height: 100px;">
                <p class="m-0">{"Unfinished Task 1"}</p>
            </div>

            <div class="border rounded p-3 d-flex justify-content-center align-items-center my-2" style="height: 100px;">
                <p class="m-0">{"Unfinished Task 2"}</p>
            </div>
        </div>
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
#[function_component]
fn AddTaskPopUp() -> Html{
    use_effect(|| {
            // SAFETY: Calling JS function after the component is mounted.
            unsafe {
                initDatePicker();
            }
            || () // no cleanup needed
        });
    //keep track of variables using oninput
    let onsubmit = Callback::from(move |form: yew::SubmitEvent| {
        
        // post request
    });
    html!{
        <div class="modal" tabindex="-1" id="exampleModal">
            <div class="modal-dialog">
              <div class="modal-content">
                <div class="modal-header">
                  <h5 class="modal-title">{"Modal title"}</h5>
                  <button type="button" class="btn-close" data-bs-dismiss="modal" aria-label="Close"></button>
                </div>
                <div class="modal-body">
                    <form onsubmit={onsubmit}>
                        <div class="form-group">
                            <label for="todotext">{"Todo-Text"}</label>
                            <textarea type="text" class="form-control" id="todotext" rows=4 placeholder="TODO"/>
                            
                        </div>
                        <div class="form-group">
                            <label for="datepicker">{"Due Date"}</label>
                            <input type="text" class="form-control" id="datepicker" placeholder="Select date"/>
                        </div>
                        <div class="form-group">
                            <label for="percenttext">{"Definition-of-done"}</label>
                            <textarea type="text" class="form-control" id="todotext" placeholder="100%"/>
                            
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