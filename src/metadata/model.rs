pub struct NewMetadata {
    pub post_type: String,
    pub companion: Option<String>,
    pub duration: Option<String>,
    pub budget_level: Option<String>,
    pub best_season: Option<String>,

    pub has_budget: bool,
    pub keywords: Vec<String>,
}

impl NewMetadata {
    pub fn new(
        post_type: &str,
        companion: Option<String>,
        duration: Option<String>,
        budget_level: Option<String>,
        best_season: Option<String>,

        has_budget: bool,
        keywords: Vec<String>,
    ) -> Self {
        Self {
            post_type: post_type.into(),
            companion,
            duration,
            budget_level,
            best_season,
            has_budget,
            keywords,
        }
    }
}

pub struct NewTheme {
    pub name: String,
    pub score: i32,
}

impl NewTheme {
    pub fn new(name: &str, score: i32) -> Self {
        Self {
            name: name.into(),
            score,
        }
    }
}

pub struct NewMentionedPlace {
    pub name: String,
    pub category: String,
    pub review_context: Option<String>,
}

impl NewMentionedPlace {
    pub fn new(name: &str, category: &str, review_context: Option<String>) -> Self {
        Self {
            name: name.into(),
            category: category.into(),
            review_context,
        }
    }
}
