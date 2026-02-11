pub struct Metadata {
    pub id: i64,
    pub article_id: i64,
    pub title: String,

    pub continent: Option<String>,
    pub country: Option<String>,
    pub city: Option<String>,

    pub country_id: Option<i16>,
    pub city_id: Option<i16>,

    pub post_type: Option<String>,
    pub companion: Option<String>,
    pub duration: Option<String>,
    pub budget_level: Option<String>,
    pub best_season: Option<String>,

    pub has_budget: bool,

    pub keywords: Vec<String>,
}

pub struct MentionedPlace {
    pub id: i32,
    pub name: String,
    pub category: String,
    pub review_context: Option<String>,
}

pub struct Theme {
    pub id: i16,
    pub name: String,
}
