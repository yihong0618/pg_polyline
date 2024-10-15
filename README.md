# pg_polyline
Fast Google Encoded Polyline encoding &amp; decoding for postgres Extension

## Usage

```sql
db=> create extension pg_polyline;
CREATE EXTENSION
db=> select polyline_encode(ARRAY[point(-120.2, 38.5), point(-120.95, 40.7), point(-126.453, 43.252)], 6);;
         polyline_encode          
----------------------------------
 _izlhA~rlgdF_{geC~ywl@_kwzCn`{nI
(1 row)

db=> select polyline_decode('_ibE_seK_seK_seK', 6);
      polyline_decode      
---------------------------
 {"(0.2,0.1)","(0.4,0.3)"}
(1 row)
```

## Installation

Assuming that rust toolchain is already istalled:

```sh
# install pgrx
cargo install --locked cargo-pgrx
cargo pgrx init
# build and install pg_polyline
git clone https://github.com/yihong0618/pg_polyline.git
cd pg_polyline
cargo pgrx run

## source pg_config then
cargo pgrx install 
```

## Kudos

- https://github.com/georust/polyline
- https://github.com/pgcentralfoundation/pgrx