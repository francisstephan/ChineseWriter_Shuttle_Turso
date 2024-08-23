use libsql::Database;
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[derive(Deserialize)]
pub struct CharData {
    pub carac: String,
}

#[derive(Deserialize)]
pub struct PinyinData {
    pub pinyin: String,
}

#[derive(Serialize)]
pub struct Zi {
    pub id: i64,
    pub pinyin_ton: String,
    pub unicode: String,
    pub hanzi: char,
    pub sens: String,
}

pub async fn getsize(client: Arc<Database>) -> i64 {
    let conn = client.connect().unwrap();
    let mut rows = conn.query("SELECT COUNT(*) FROM pyhz", ()).await.unwrap();
    let Some(row) = rows.next().await.unwrap() else {
        return 0;
    };
    row.get::<i64>(0).unwrap()
}

pub async fn list_for_zi(client: Arc<Database>, first: String) -> Vec<Zi> {
    let conn = client.connect().unwrap();
    let whereclause = format!(" unicode = '{}' ORDER BY pinyin_ton", &first);
    readdic(conn, &whereclause).await
}

pub async fn readdic(conn: libsql::Connection, whereclause: &str) -> Vec<Zi> {
    let basequery = "SELECT id, pinyin_ton, unicode, sens FROM pyhz";
    let qq: String;
    let query = if !whereclause.is_empty() {
        qq = format!("{} WHERE {}", basequery, whereclause);
        &qq
    } else {
        basequery
    };
    read_query(conn, query).await
}

async fn read_query(conn: libsql::Connection, query: &str) -> Vec<Zi> {
    let mut disp = Vec::<Zi>::new();

    let mut rows = conn.query(query, ()).await.unwrap();
    let mut unread = true;
    while unread {
        match rows.next().await {
            Ok(res) => match res {
                Some(row) => {
                    let unicode = row.get::<String>(2).unwrap();
                    let unicodestr = u32::from_str_radix(unicode.as_str(), 16).unwrap();
                    let carac = char::from_u32(unicodestr).unwrap();
                    let zi = Zi {
                        id: row.get::<i64>(0).unwrap(),
                        pinyin_ton: row.get::<String>(1).unwrap().clone(),
                        unicode: unicode.clone(),
                        hanzi: carac,
                        sens: row.get::<String>(3).unwrap().clone(),
                    };
                    disp.push(zi);
                }
                None => {
                    unread = false;
                }
            },
            Err(_) => {
                unread = false;
            }
        }
    }

    disp
}

pub async fn list_for_py(client: Arc<Database>, chaine: String) -> Vec<Zi> {
    let conn = client.connect().unwrap();
    let last_char = &chaine.chars().last().unwrap();
    let cond = matches!(last_char, '0'..='4');
    let whereclause = if !cond {
        // no tone given: check all tones 0 to 4
        format!(" pinyin_ton = '{}0' OR pinyin_ton = '{}1' OR pinyin_ton = '{}2' OR pinyin_ton = '{}3' OR pinyin_ton = '{}4' ORDER BY pinyin_ton, unicode"
            , &chaine,&chaine,&chaine,&chaine,&chaine )
    } else {
        format!(" pinyin_ton = '{}' ORDER BY unicode", &chaine)
    };
    readdic(conn, &whereclause).await
}
