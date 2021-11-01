#[derive(Insertable)]
#[table_name="songs"]

struct Song {
    name: String,
    artist: String,
};
