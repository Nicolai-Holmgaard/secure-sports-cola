use crate::api::client::{
    get_active_products, get_member_balance, get_member_id, get_named_products, post_sale,
};
use crate::api::types::SaleRequest;
use clap::Parser;
mod api;
mod cli;

#[tokio::main]
async fn main() -> Result<(), confy::ConfyError> {
    let cli = cli::CliOptions::parse();
    let mut cfg: cli::SSCConfig = confy::load("secure-sports-cola", "config")?;

    let username = cli.username.unwrap_or(cfg.username.clone());
    if cfg.username.is_empty() {
        if username.is_empty() {
            eprintln!(
                "Username must be provided either through the --username argument or the config file."
            );
            std::process::exit(1);
        }

        cfg.username = username.clone();
        confy::store("secure-sports-cola", "config", &cfg)?;
    }
    let member_id = get_member_id(&cfg.url, &username).await.unwrap();

    if cli.balance {
        let balance = get_member_balance(&cfg.url, &member_id).await.unwrap();
        println!("Balance: {}", balance);
        return Ok(());
    }
    if cli.list {
        let products = get_active_products(&cfg.url, cfg.room).await.unwrap();
        let named_products = get_named_products(&cfg.url).await.unwrap();
        println!("{:#?}", named_products);
        println!("Active products:");
        for (id, product) in products {
            let shorts = named_products
                .iter()
                .filter(|named_product| named_product.1.to_string() == id);
            println!(
                "{:4} {} {:7} {}",
                id,
                shorts.count(),
                format!("({})", product.price),
                product.name
            );
        }
        return Ok(());
    }

    let SaleReq = SaleRequest {
        member_id,
        room: cfg.room,
        buystring: format!("{} {}", &username, cli.buystring.join(" ")),
    };

    println!("{:?}", SaleReq);

    post_sale(&cfg.url, SaleReq).await.unwrap();
    Ok(())
}
