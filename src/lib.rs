pub fn add(left: usize, right: usize) -> usize {
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = add(2, 2);
        assert_eq!(result, 4);
    }
}

struct TodoApp {
    link: ComponentLink<Self>,
    todos: Option<Vec<Todo>>,
    fetch_task: Option<FetchTask>,
}

#[derive(Deserialize, Clone, PartialEq, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Todo {
pub user_id: u64,
pub id: u64,
pub title: String,
pub completed: bool,
}

enum Msg {
    MakeReq,
    Resp(Result<Vec<Todo>, anyhow::Error>),
}

impl Component for TodoApp {
    type Message = Msg;
    type Properties = ();
    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        link.send_message(Msg::MakeReq);
        Self {
            link,
            todos: None,
            fetch_task: None,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::MakeReq => {
                self.todos = None;
                let req = Request::get("https://jsonplaceholder.typicode.com/todos")
                    .body(Nothing)
                    .expect("can make req to jsonplaceholder");

                let cb = self.link.callback(
                    |response: Response<Json<Result<Vec<Todo>, anyhow::Error>>>| {
                        let Json(data) = response.into_body();
                        Msg:Resp(data)
                    },
                );

                let task = FetchService::fetch(req, cb).expect("can create task")
                self.fetch_task = Some(task);
                ()
            }
            Msg::Resp(resp) => {
                if let Ok(data) = resp {
                    self.todos = Some(data);
                }                
            }
        }
        true
    }

    fn change(&mut self, _props: Self::Properties) -> ShouldRender {
        false
    }

    fn view(&Self) -> Html {
        let todos = self.todos.clone();
        let cb = self.link.callback(|_| Msg::MakeReq);
        ConsoleService::info(&format!("render TodoApp: {:?}", todos));
        html! {
            <div class=classes!("todo")>
                <div>
                    <div class=classes!("refresh")>
                        <button onclick=cb.clone()>
                            {"refresh"}
                        </button>
                    </div>
                    <todo::list::List todos=todos.clone()/>
                </div>
            </div>
        }
    }
}

#[wasm_bindgen(start)]
pub fn run_app() {
    App::<TodoApp>::new().mount_to_body(;
}