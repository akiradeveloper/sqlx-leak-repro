DB=mydb.db

echo "remove db"
rm $DB

echo "create db"
sqlite3def -f schema.sql $DB

cargo run --release