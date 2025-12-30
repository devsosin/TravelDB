use crate::{
    RepositoryResult,
    metadata::{
        MetaRepositoryImpl,
        entity::MentionedPlace,
        model::{NewMentionedPlace, NewTheme},
    },
};

pub trait MetaMentionedPlaceRepository {
    fn save_places(
        &self,
        meta_id: i64,
        new_places: Vec<NewMentionedPlace>,
    ) -> impl Future<Output = RepositoryResult<Vec<i32>>>;
}

impl MetaMentionedPlaceRepository for MetaRepositoryImpl {
    async fn save_places(
        &self,
        meta_id: i64,
        new_places: Vec<NewMentionedPlace>,
    ) -> RepositoryResult<Vec<i32>> {
        let names = new_places
            .iter()
            .map(|p| p.name.clone())
            .collect::<Vec<String>>();
        let categories = new_places
            .iter()
            .map(|p| p.category.clone())
            .collect::<Vec<String>>();
        let reviews = new_places
            .iter()
            .map(|p| p.review_context.clone())
            .collect::<Vec<_>>();

        let records = sqlx::query!(
            r#"
            WITH PlaceDatas AS (
                SELECT name, category, review_context
                FROM UNNEST($2::text[], $3::text[], $4::text[])
                    AS u(name, category, review_context)
            ),
            InsertedPlaces AS (
                INSERT INTO tb_mentioned_place(name, category, review_context)
                    SELECT name, category, review_context
                    FROM PlaceDatas
                RETURNING Id
            )
            INSERT INTO tb_meta_place(metadata_id, place_id)
                SELECT $1, id
                FROM InsertedPlaces
            RETURNING place_id
            "#,
            meta_id,
            &names,
            &categories,
            reviews as _,
        )
        .fetch_all(&self.pool)
        .await?;

        let ids = records.iter().map(|r| r.place_id).collect();

        Ok(ids)
    }
}

pub trait MetaThemeRepository {
    fn save_themes(
        &self,
        meta_id: i64,
        new_themes: Vec<NewTheme>,
    ) -> impl Future<Output = RepositoryResult<Vec<i64>>>;
}

impl MetaThemeRepository for MetaRepositoryImpl {
    async fn save_themes(
        &self,
        meta_id: i64,
        new_themes: Vec<NewTheme>,
    ) -> RepositoryResult<Vec<i64>> {
        let names = new_themes
            .iter()
            .map(|t| t.name.clone())
            .collect::<Vec<String>>();
        let scores = new_themes.iter().map(|t| t.score).collect::<Vec<i32>>();

        let records = sqlx::query!(
            r#"
            WITH ThemeDatas AS (
                SELECT t.id, u.name, score 
                    FROM UNNEST($2::text[], $3::int[]) AS u(name, score)
                    LEFT JOIN tb_theme AS t ON t.name = u.name
            ),
            InsertedThemes AS (
                INSERT INTO tb_theme(name)
                    SELECT name
                    FROM ThemeDatas
                    WHERE id IS NULL
                RETURNING id, name
            )
            INSERT INTO tb_theme_meta(theme_id, metadata_id, score)
                SELECT 
                    CASE 
                        WHEN id IS NULL THEN (SELECT id FROM InsertedThemes AS i WHERE i.name = t.name)
                        ELSE id
                    END, $1, score
                FROM ThemeDatas AS t
            RETURNING id
            "#,
            meta_id,
            &names,
            &scores,
        )
        .fetch_all(&self.pool)
        .await?;

        let ids = records.iter().map(|r| r.id).collect();

        Ok(ids)
    }
}
