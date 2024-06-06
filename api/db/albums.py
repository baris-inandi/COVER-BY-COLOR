import sqlite3

con = sqlite3.connect("albums.sql")

con.execute("SELECT * FROM Albums")
