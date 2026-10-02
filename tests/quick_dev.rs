use anyhow::Ok;
use anyhow::Result;

use serde_json::json;

#[derive(serde::Deserialize)]
struct LoginResponse {
    token: String,
}

#[tokio::test]
async fn quick_dev() -> Result<()> {
    let hc = httpc_test::new_client("http://localhost:5000")?;

    println!("==== login =====");

    let login: LoginResponse = hc
        .post(
            "/api/auth/login",
            json!({
                "email": "github@.com",
                "password": "my-password",
            }),
        )
        .await?;

    println!("Token received: {}", login.token);

    println!("==== posturl=====\n\n");

    /*let post = hc
        .reqwest_client()
        .post("http://localhost:5000/api/url")
        .json(&json!({
            "url":"https://rust-lang.org/learn/"
        }))
        .header("Authorization", format!("Bearer {}", login.token))
        .send()
        .await?;

    println!("POST status: {}", post.status());
    println!("POST body: {}", post.text().await?);

    println!("==== delete url=====\n\n");*/

    let delete = hc
        .reqwest_client()
        .delete("http://localhost:5000/api/urls/005f3e5d-9d5c-4f13-ba21-cfe672b99af3")
        .header("Authorization", format!("Bearer {}", login.token))
        .send()
        .await?;
    println!("delete Status:{}", delete.status());
    println!("Delete body:{}", delete.text().await?);

    println!("==== users=====\n\n");

    hc.do_get("/users").await?.print().await?;

    println!("==== redirect url=====\n\n");

    /*let redirect = hc
        .reqwest_client()
        .get("http://localhost:5000/czEcQk")
        .header("Authorization", format!("Bearer {}", login.token))
        .send()
        .await?;

    println!("redirect Status:{}", redirect.status());
    println!("redirect body:{}", redirect.text().await?);*/

    println!("==== url_clics_count=====\n\n");
    hc.do_get("/urlclicks").await?.print().await?;

    println!("=========url analysis=====\n\n");
    let analysis = hc
        .reqwest_client()
        .get("/api/urls/f102538c-b081-450f-ba42-25aa40afb66e/analytics")
        .header("Authorization", format!("Bearer {}", login.token))
        .send()
        .await?;
    println!("analysis Status:{}", analysis.status());
    println!("analysis body:{}", analysis.text().await?);

    println!("==== response status=====\n\n");

    let response = hc
        .reqwest_client()
        .get("http://localhost:5000/api/urls")
        .header("Authorization", format!("Bearer {}", login.token))
        .send()
        .await?;

    println!("Status: {}", response.status());
    println!("Body: {}", response.text().await?);

    Ok(())
}
