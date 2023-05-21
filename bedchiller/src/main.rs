#[derive(Debug)]
enum Input {
    Temp(i32),
    Points(Vec<(i32, i32)>),
    PowerOff,
}

async fn get_updated_app_input() -> Input {
    Input::Points(vec![(1, 2), (2, 3)])
}

async fn get_updated_button_input() -> Input {
    Input::Temp(5)
}

async fn power_off() -> Input {
    Input::PowerOff
}

#[tokio::main]
async fn main() {
    loop {
        tokio::select! {
            data = get_updated_app_input() => {
                println!("{data:?}")
            }
            data = get_updated_button_input() => {
                println!("{data:?}")
            }
            _ = power_off() => {
                println!("Powering Off!");
                break
            }
        }
    }
}
