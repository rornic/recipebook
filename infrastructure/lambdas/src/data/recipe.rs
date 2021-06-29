use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Serialize, Deserialize, Default)]
pub struct Recipe {
    pub uuid: Uuid,
    title: Option<String>,
    description: Option<String>,
    ingredients: Option<Vec<String>>,
    method: Option<Vec<String>>,
    image: Option<String>,
    author: Option<String>,
    servings: Option<u16>,
}

impl Recipe {
    pub fn new(
        title: &str,
        description: &str,
        ingredients: Vec<String>,
        method: Vec<String>,
        image: &str,
        author: &str,
        servings: u16,
    ) -> Recipe {
        let uuid = Uuid::new_v4();
        Recipe {
            uuid: uuid,
            title: Some(title.to_owned()),
            description: Some(description.to_owned()),
            ingredients: Some(ingredients),
            method: Some(method),
            image: Some(image.to_owned()),
            author: Some(author.to_owned()),
            servings: Some(servings),
        }
    }
}

pub fn validate_recipe(recipe: &mut Recipe) {
    match &recipe.title {
        Some(x) => {
            if x.is_empty() {
                recipe.title = Some("Untitled".to_string());
            }
        }
        None => {
            recipe.title = Some("Untitled".to_string());
        }
    }

    match &recipe.author {
        Some(x) => {
            if x.is_empty() {
                recipe.author = Some("Unknown".to_string());
            }
        }
        None => {
            recipe.author = Some("Unknown".to_string());
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::collections::HashSet;
    #[test]
    fn test_uuid_length() {
        let recipe = Recipe::new("", "", Vec::new(), Vec::new(), "", "", 4);
        println!("Testing UUID {}", recipe.uuid);
        assert_eq!(recipe.uuid.to_string().len(), 36);
    }

    #[test]
    fn test_uuid_uniqueness() {
        let mut recipes = HashSet::new();

        (0..100).into_iter().for_each(|_| {
            assert_eq!(
                true,
                recipes.insert(Recipe::new("", "", Vec::new(), Vec::new(), "", "", 4).uuid)
            )
        });
    }

    #[test]
    fn test_recipe_title_validation() {
        let mut recipe = Recipe {
            title: None,
            author: Some("Author".to_string()),
            ..Default::default()
        };
        validate_recipe(&mut recipe);

        let mut recipe2 = Recipe {
            title: Some("".to_string()),
            author: Some("Author".to_string()),
            ..Default::default()
        };
        validate_recipe(&mut recipe2);

        assert_eq!(recipe.title, Some("Untitled".to_string()));
        assert_eq!(recipe2.title, Some("Untitled".to_string()));
    }

    #[test]
    fn test_recipe_author_validation() {
        let mut recipe = Recipe {
            title: Some("Title".to_string()),
            author: None,
            ..Default::default()
        };
        validate_recipe(&mut recipe);
        let mut recipe2 = Recipe {
            title: Some("Title".to_string()),
            author: Some("".to_string()),
            ..Default::default()
        };
        validate_recipe(&mut recipe2);

        assert_eq!(recipe.author, Some("Unknown".to_string()));
        assert_eq!(recipe2.author, Some("Unknown".to_string()));
    }
}
