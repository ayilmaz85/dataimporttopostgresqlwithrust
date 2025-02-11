use crate::model::movie_model::MovieRow;
use postgres::{Client, NoTls};
use std::error::Error;

pub struct DBHandler {
    pub client: Client,
}

impl DBHandler {
    /// Yeni bir DBHandler oluştur ve PostgreSQL'e bağlan
    pub fn new(connection_string: &str) -> Result<Self, Box<dyn Error>> {
        let client = Client::connect(connection_string, NoTls)?;
        Ok(DBHandler { client })
    }

    /// Veritabanındaki tabloları oluştur
    pub fn create_tables(&mut self) -> Result<(), Box<dyn Error>> {
        self.client.batch_execute(
            "
            CREATE TABLE IF NOT EXISTS movies (
                id SERIAL PRIMARY KEY,
                title VARCHAR NOT NULL,
                original_title VARCHAR,
                imdb_id VARCHAR UNIQUE,
                overview TEXT,
                tagline VARCHAR,
                release_date DATE,
                status VARCHAR,
                runtime INTEGER,
                budget BIGINT,
                revenue BIGINT,
                homepage VARCHAR,
                adult BOOLEAN,
                popularity DECIMAL,
                vote_average DECIMAL,
                vote_count INTEGER,
                backdrop_path VARCHAR,
                poster_path VARCHAR,
                original_language VARCHAR
            );

            CREATE TABLE IF NOT EXISTS genres (
                id SERIAL PRIMARY KEY,
                name VARCHAR UNIQUE NOT NULL
            );

            CREATE TABLE IF NOT EXISTS movie_genres (
                movie_id INTEGER REFERENCES movies(id) ON DELETE CASCADE,
                genre_id INTEGER REFERENCES genres(id) ON DELETE CASCADE,
                PRIMARY KEY (movie_id, genre_id)
            );
            ",
        )?;
        Ok(())
    }

    /// Verilen kategoride ismi ekle veya mevcut ID'yi döndür
    pub fn get_or_create_id(&mut self, name: &str, table: &str) -> Result<i32, Box<dyn Error>> {
        let rows = self.client.query(
            &format!(
                "
                INSERT INTO {} (name)
                VALUES ($1)
                ON CONFLICT (name) DO UPDATE SET name = EXCLUDED.name
                RETURNING id",
                table
            ),
            &[&name],
        )?;

        let id: i32 = rows[0].get(0);
        Ok(id)
    }

    /// Ana film bilgilerini veritabanına ekle

    pub fn insert_movie(&mut self, movie: &MovieRow) -> Result<(), Box<dyn Error>> {
        self.client.execute(
            "INSERT INTO movies (
            id, title, original_title, imdb_id, overview, tagline, release_date, status,
            runtime, budget, revenue, homepage, adult, popularity, vote_average, vote_count,
            backdrop_path, poster_path, original_language
        ) VALUES ($1, $2, $3, $4, $5, $6, $7, $8, $9, $10, $11, $12, $13, $14, $15, $16, $17, $18, $19)
        ON CONFLICT (id) DO NOTHING",
            &[
                &movie.id,
                &movie.title,
                &movie.original_title,
                &movie.imdb_id,
                &movie.overview,
                &movie.tagline,
                &movie.release_date.map(|d| d.format("%Y-%m-%d").to_string()).as_deref(), // ✅ DÜZELTME: `Option<NaiveDate>` → `Option<&NaiveDate>`
                &movie.status,
                &movie.runtime,
                &movie.budget,
                &movie.revenue,
                &movie.homepage,
                &movie.adult,
                &movie.popularity,
                &movie.vote_average,
                &movie.vote_count,
                &movie.backdrop_path,
                &movie.poster_path,
                &movie.original_language,
            ],
        )?;
        Ok(())
    }
}
