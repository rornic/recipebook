use async_trait::async_trait;
use rusoto_core::Region;
use rusoto_dynamodb::{
    DeleteItemInput, DynamoDb, DynamoDbClient, GetItemInput, PutItemInput, ScanInput,
};
use serde::Serialize;

use crate::data::recipe::{validate_recipe, Recipe};

#[async_trait]
pub trait Db {
    async fn get_all_recipes(&self) -> Result<Vec<Recipe>, String>;
    async fn get_recipe(&self, uuid: String) -> Result<Recipe, String>;
    async fn update_recipe(&self, uuid: String, recipe: &mut Recipe) -> Result<(), String>;
    async fn delete_recipe(&self, uuid: String) -> Result<(), String>;
    async fn create_recipe(&self) -> Result<Recipe, String>;
}

const REGION: Region = Region::EuWest2;

pub struct Dynamo {
    table: String,
    client: DynamoDbClient,
}

impl Dynamo {
    pub fn new() -> Dynamo {
        let client: DynamoDbClient = DynamoDbClient::new(REGION);
        Dynamo {
            client,
            table: String::from("recipebook-recipes"),
        }
    }
}

#[async_trait]
impl Db for Dynamo {
    async fn get_all_recipes(&self) -> Result<Vec<Recipe>, String> {
        match self
            .client
            .scan(ScanInput {
                table_name: self.table.clone(),
                ..Default::default()
            })
            .await
        {
            Ok(scan_output) => {
                if let Some(items) = scan_output.items {
                    Ok(items
                        .into_iter()
                        .map(|i| {
                            serde_dynamodb::from_hashmap::<Recipe, _>(i)
                                .expect("Could not convert HashMap to Recipe")
                        })
                        .collect::<Vec<Recipe>>())
                } else {
                    Ok(vec![])
                }
            }
            Err(e) => Err(format!("Could not retrieve recipes: {}", e.to_string())),
        }
    }

    async fn get_recipe(&self, uuid: String) -> Result<Recipe, String> {
        #[derive(Serialize)]
        struct Key {
            uuid: String,
        }
        let key = Key { uuid: uuid.clone() };

        match self
            .client
            .get_item(GetItemInput {
                table_name: self.table.clone(),
                key: serde_dynamodb::to_hashmap(&key).map_err(|err| {
                    format!("Could not convert query to HashMap: {}", err.to_string())
                })?,
                ..Default::default()
            })
            .await
        {
            // Query went OK, deserialize the response
            Ok(item_output) => {
                if let Some(item) = item_output.item {
                    let recipe: Recipe = serde_dynamodb::from_hashmap(item).map_err(|err| {
                        format!("Could not convert HashMap to recipe: {}", err.to_string())
                    })?;

                    Ok(recipe)
                } else {
                    Err(format!("No items found with uuid {}", uuid))
                }
            }
            // Error with query
            Err(e) => Err(format!(
                "Error executing GetItem query for recipe {}: {}",
                uuid, e
            )),
        }
    }

    async fn update_recipe(&self, uuid: String, recipe: &mut Recipe) -> Result<(), String> {
        validate_recipe(recipe);
        match self
            .client
            .put_item(PutItemInput {
                table_name: self.table.clone(),
                item: serde_dynamodb::to_hashmap(recipe)
                    .map_err(|err| format!("Could not convert recipe to HashMap: {}", err))?,
                ..Default::default()
            })
            .await
        {
            Ok(_) => Ok(()),
            Err(e) => Err(format!(
                "Error executing PutItem query for recipe {}: {}",
                uuid, e
            )),
        }
    }

    async fn delete_recipe(&self, uuid: String) -> Result<(), String> {
        #[derive(Serialize)]
        struct Key {
            uuid: String,
        }
        let key = Key { uuid: uuid.clone() };
        match self
            .client
            .delete_item(DeleteItemInput {
                table_name: self.table.clone(),
                key: serde_dynamodb::to_hashmap(&key).map_err(|err| {
                    format!("Could not convert query to HashMap: {}", err.to_string())
                })?,
                ..Default::default()
            })
            .await
        {
            Ok(_output) => Ok(()),
            Err(e) => Err(format!("Could not delete recipe {}: {}", uuid, e)),
        }
    }

    async fn create_recipe(&self) -> Result<Recipe, String> {
        let mut recipe = Recipe::new(
            "Untitled",
            "My new recipe.",
            vec!["".to_string()],
            vec!["".to_string()],
            "https://www.generationsforpeace.org/wp-content/uploads/2018/03/empty.jpg",
            "Unknown",
            4,
        );
        validate_recipe(&mut recipe);

        match self
            .client
            .put_item(PutItemInput {
                table_name: self.table.clone(),
                item: serde_dynamodb::to_hashmap(&recipe)
                    .map_err(|err| format!("Could not convert recipe to HashMap: {}", err))?,
                ..Default::default()
            })
            .await
        {
            Ok(_) => Ok(recipe),
            Err(e) => Err(format!("Could not create new recipe: {}", e)),
        }
    }
}
