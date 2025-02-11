mod model;
use tokio_postgres::types::ToSql;
use crate::model::movie_model::Movie;
use chrono::NaiveDate;
use csv::ReaderBuilder;
use std::error::Error;
use rust_decimal::Decimal;
use tokio_postgres::{NoTls};


///
/// önce çalışan bir version oluşturdum
/// sonrasında best practice'lere uygun hale getireceğim.
///insert fonksiyonunu
/// csv okuma fonksiyonunu main dışına çıkaracağım
/// db connect işlemini ve geriye connection döndürme işlemini
/// farklı bir modüle alacağım
///
///

#[tokio::main]
async fn main() -> Result<(), Box<dyn Error>> {
    //client nesnesini oluştur
    ///TO DO: Constant olarak ya da yaml olarak tanımla ve oradan oku
    let (client, connection) = tokio_postgres::connect(
        "host=192.168.1.112 user=postgres dbname=TmdbFilmset password=aedes",
        NoTls,
    )
        .await?;
//Connection sağlanamazsa hata fırlatır
    tokio::spawn(async move {
        if let Err(e) = connection.await {
            eprintln!("Connection error: {}", e);
        }
    });
//csv dosyasında veri okur
    let mut rdr = ReaderBuilder::new()
        .has_headers(true)
        .from_path("src/data/TMDB.csv")?;
// insert query si
    ///
    /// TO DO: Database normalizasyonu db üzerinden sqllerle yapılacak
    ///
    let stmt = "INSERT INTO moviestest (id,title, vote_average, \
    vote_count, status, release_date, revenue, runtime, adult, backdrop_path, \
    budget, homepage, imdb_id, original_language, original_title, overview, \
    popularity, poster_path, tagline, genres, production_companies, \
    production_countries, spoken_languages, keywords) VALUES ($1, $2, $3, $4, $5, $6, \
    $7, $8, $9, $10, $11, $12, \
    $13, $14, $15, $16, $17, $18, $19, $20, $21, $22,$23,$24) ON CONFLICT (id) DO NOTHING";
//csv reader'ı dolaşıyoruz
    for result in rdr.records() {

        ///burada reader'dan okuduğum verilerde inceleme
        /// yaptım. bazılarının kolon sayılarının eksik olduğunu gördüm
        /// eksik kolonları db ile uyumlu olması için 24 e tamamladım
        let mut record = result?.iter().map(|s| s.to_string()).collect::<Vec<String>>();
        if record.len() < 24 {
            record.resize(24, "".to_string()); // Eksik sütunları "" ile doldur
        }

        ///Model nesnesinden bir instance oluşturdum
        /// unwrap_or default değer atayabilmek için kullandım.
        /// unwrap ediyorum yoksa default bir değer atıyorum
        let movie = Movie {
           id: record[0].parse()?,
           title: record[1].to_string(),
            vote_average: record[2].parse().unwrap_or(Decimal::ZERO).round_dp(2),
            vote_count: record[3].parse().unwrap_or(0),
            status: record[4].to_string(),
            release_date: if record[5].is_empty() {
                None
            }else {
                Some(NaiveDate::parse_from_str(&record[5], "%Y-%m-%d")?,)
            }
            ,revenue: record[6].parse().unwrap_or(0),
            runtime: record[7].parse().unwrap_or(0),
            adult: record[8].to_lowercase().parse().unwrap_or(false),
            backdrop_path: record[9].to_string(),
            budget: record[10].parse().unwrap_or(0),
            homepage: record[11].to_string(),
            imdb_id: record[12].to_string(),
            original_language: record[13].to_string(),
            original_title: record[14].to_string(),
            overview: record[15].to_string(),
            popularity: record[16].parse().unwrap_or(Decimal::ZERO).round_dp(6),
            poster_path: record[17].to_string(),
            tagline: record[18].to_string(),
            genres: record[19].to_string(),
            production_companies: record[20].to_string(),
            production_countries: record[21].to_string(),
            spoken_languages: record[22].to_string(),
            keywords: record[23].to_string(),
        };
        //println!("{}", movie.title);
        //print_type_of(&movie.release_date);

       ///burada insert ediyorum
       /// match ile aldım hata alırsam ona göre result dönmek için
        match client
            .execute(
                stmt,
                &[
                    &movie.id as &(dyn ToSql + Sync),
                    &movie.title,
                    &movie.vote_average as &(dyn ToSql + Sync),
                    &movie.vote_count as &(dyn ToSql + Sync),
                    &movie.status,
                    &movie.release_date.as_ref()  as &(dyn ToSql + Sync),
                    &movie.revenue as &(dyn ToSql + Sync),
                    &movie.runtime as &(dyn ToSql + Sync),
                    &movie.adult as &(dyn ToSql + Sync),
                    &movie.backdrop_path,
                    &movie.budget as &(dyn ToSql + Sync),
                    &movie.homepage,
                    &movie.imdb_id,
                    &movie.original_language,
                    &movie.original_title,
                    &movie.overview,
                    &movie.popularity as &(dyn ToSql + Sync),
                    &movie.poster_path,
                    &movie.tagline,
                    &movie.genres,
                    &movie.production_companies,
                    &movie.production_countries,
                    &movie.spoken_languages,
                    &movie.keywords,
                ],
            ).await {
            Ok(_) => println!("Inserted: {}", movie.title),
            Err(e) => eprintln!("DB Insert Error: {}", e),
        }


    }




    Ok(())
}

