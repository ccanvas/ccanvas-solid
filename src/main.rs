use libccanvas::{
    bindings::{Colour, EventVariant, Subscription},
    client::{Client, ClientConfig},
    features::common::Dimension,
};

#[tokio::main]
async fn main() {
    let client = Client::new(ClientConfig::default()).await;

    let (_, term_size) = tokio::join!(
        client.subscribe(Subscription::ScreenResize),
        client.term_size()
    );

    let mut term_size: Dimension = term_size.into();
    let c = std::env::var("CHAR")
        .map(|c| c.chars().next().unwrap_or('#'))
        .unwrap_or('#');

    render(&client, term_size, c);
    client.renderall().await;

    loop {
        let event = client.recv().await;

        if let EventVariant::Resize { width, height } = event.get() {
            term_size = Dimension::new(*width, *height);
            render(&client, term_size, c);
            client.renderall().await;
        }
    }
}

pub fn render(client: &Client, dim: Dimension, c: char) {
    for y in 0..dim.height {
        for x in 0..dim.width {
            client.setcharcoloured(x, y, c, Colour::rgb(100, 100, 100), Colour::Reset)
        }
    }
}
