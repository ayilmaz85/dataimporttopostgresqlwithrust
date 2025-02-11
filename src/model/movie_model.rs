use chrono::NaiveDate;
use rust_decimal::Decimal;

#[derive(Debug)]
pub struct Movie {
    pub id: i64,
    pub title: String,
    pub vote_average: Decimal,
    pub vote_count: i64,
    pub status: String,
    pub release_date: Option<NaiveDate>,
    pub revenue: i64,
    pub runtime: i32,
    pub adult: bool,
    pub backdrop_path: String,
    pub budget: i64,
    pub homepage: String,
    pub imdb_id: String,
    pub original_language: String,
    pub original_title: String,
    pub overview: String,
    pub popularity: Decimal,
    pub poster_path: String,
    pub tagline: String,
    pub genres: String,
    pub production_companies: String,
    pub production_countries: String,
    pub spoken_languages: String,
    pub keywords: String,
}
