# PGScraper

This is a small extension to allow you to scrape data from web directly from
your postgres database.

This is a pet project I'm working to learn Rust.


A few useful commands:

To init your [pgx](https://github.com/zombodb/pgx) and provide some instances
only to test the extension, use it only once:

```
cargo pgx init
```

After cloning, you can install the extension:

```
cargo pgx install
```

Run on Postgres13:

```
cargo pgx run pg13
```

Then, on psql you'll need to enable the extension:

```
CREATE EXTENSION pgscraper;
```

And now you have access to two functions:

```sql
select html_select('title', 'https://blog.timescale.com');
┌───────────────────────────────┐
│          html_select          │
├───────────────────────────────┤
│ <title>Timescale Blog</title> │
└───────────────────────────────┘
(1 row)

```
Or just internal text:

```sql
 select html_select_text('title', 'https://blog.timescale.com');
┌──────────────────┐
│ html_select_text │
├──────────────────┤
│ Timescale Blog  ↵│
│                  │
└──────────────────┘
(1 row)
```

This project is inspired on the
[Bad Postgres extension ideas](https://m.twitch.tv/videos/694514963)!
