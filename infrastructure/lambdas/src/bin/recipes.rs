use http::Method;
use lambda_http::{handler, IntoResponse, Request, RequestExt, Response};
use lambda_runtime::{self, Context, Error};
use lib::data::recipe::Recipe;
use lib::db::{Db, Dynamo};
use lib::ErrorResponse;
use serde_json::json;
use serde_json::Value;

#[tokio::main]
async fn main() -> Result<(), Error> {
    lambda_runtime::run(handler(func)).await?;
    Ok(())
}

async fn func(event: Request, _: Context) -> Result<impl IntoResponse, Error> {
    let db = Dynamo::new();

    let value = match event.method() {
        &Method::POST => handle_post(db).await,
        &Method::GET => handle_get(db, event).await,
        &Method::PUT => handle_put(db, event).await,
        &Method::DELETE => handle_delete(db, event).await,
        x => Err(format!("Unsupported method {}", x)),
    };

    match value {
        Ok(val) => Ok(Response::builder()
            .status(200)
            .header("Access-Control-Allow-Origin", "*")
            .body(serde_json::to_string(&val)?)?),
        Err(val) => Ok(Response::builder()
            .status(500)
            .header("Access-Control-Allow-Origin", "*")
            .body(serde_json::to_string(&ErrorResponse { error: val })?)?),
    }
}

async fn handle_post(db: Dynamo) -> Result<Value, String> {
    let recipe = db.create_recipe().await?;
    Ok(json!(recipe))
}

async fn handle_get(db: Dynamo, event: Request) -> Result<Value, String> {
    match event.path_parameters().get("recipe_id") {
        // uuid specified, get specific recipe
        Some(uuid) => {
            let recipe = db.get_recipe(uuid.to_string()).await?;
            Ok(json!(recipe))
        }
        // No uuid, get all recipes
        None => {
            let recipes = db.get_all_recipes().await?;
            Ok(json!(recipes))
        }
    }
}

async fn handle_delete(db: Dynamo, event: Request) -> Result<Value, String> {
    match event.path_parameters().get("recipe_id") {
        // uuid specified, delete recipe
        Some(uuid) => {
            db.delete_recipe(uuid.to_string()).await?;
            Ok(json!(""))
        }
        // No uuid, error
        None => Err(format!("Error deleting recipe: No uuid supplied.")),
    }
}

async fn handle_put(db: Dynamo, event: Request) -> Result<Value, String> {
    // Read recipe from body
    let mut recipe = read_body_as_recipe(event.body())?;
    db.update_recipe(recipe.uuid.to_string(), &mut recipe)
        .await?;
    Ok(json!(recipe))
}

fn read_body_as_recipe(body: &[u8]) -> Result<Recipe, String> {
    let recipe = serde_json::from_slice::<Recipe>(body)
        .map_err(|err| format!("Could not deserialize body as recipe: {}", err))?;
    Ok(recipe)
}
