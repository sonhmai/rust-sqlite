# Roadmap and TODO

## Roadmap
1. Implement an MVP version for running TPH benchmark read queries.
2. Benchmark
3. Improve
4. Implement an MVP version for running TPH benchmark transaction queries.
5. Benchmark
6. Improve
7. Rinse and Repeat


## CLI
- [ ] implement .dbinfo command. Pass `tests/test_dot_commands.rs cli_dbinfo` test.


## Query Planning and Processing (Read Path)

Scan
- [x] Implement DbHeader
- [x] SqliteContextProvider: parse db header from file for tables and schemas
  - [x] Create DiskManager for centralizing physical ops instead of using BufferPool.
  - [x] Centralize init process in Database.
  - [x] Implement DbMeta to be able to parse DbHeader and schema objects
    - [x] Page cell_ptrs()
    - [x] DbMeta parses leaf_table_cells for first page
    - [x] Implement SchemaObject::parse(&LeafTableCell)
    - [x] Add parsing Columns from sql statement to SchemaObject::parse
    - [x] Implement field DbMeta.schema_objects
  - [x] Implement this conversion in SqliteContextProvider::new. From schema objects we can get table (name, cols, data types) for SqliteContextProvider
  potentially we need to convert sqlite type to arrow_schema types.
- [ ] ExecScan: implement Physical TableScan
  - [ ] Implement one page scan
  - [ ] PhysicalPlanner.plan(): Replace hardcoded ExecApplesScan with actual Scan.
- [ ] Scan Many B+tree pages
  - [ ] implement dfs or bfs for scanning across multiple pages.

Projection
- [x] select * from table1
- [ ] Implement Project by Column Name in ExecProjection: select col1 from table1
- [ ] select col1, col2 from table1

Selection
- [ ] Where `select col1 from table1 where col2='value'`
- [ ] Where `IN`

Aggregation
- [ ] Count: `select name, count(1) from apples group by name;`
- [ ] Max: `select name, max(color) from apples group by name;`
- [ ] Average

Component
- [x] basic SQL to Logical Plan
- [x] basic Logical Plan execution
- [x] physical plan
- [ ] ColumnValue and DataRecord
- [ ] Parsing database
- [ ] Parsing table
- [ ] replace hardcoded ExecApplesScan by actual sqlite table scan


## Query Optimizer
TODO, not prioritized yet.


## BTree Module

```
Layering

ExecScan (PhysicalPlan)
---
BTree Module
---
BufferPool

```

where is it?
- above BufferPool, calls by PhysicalPlan (i.e. ExecScan) to scan
all table pages when reading from table (i.e. `select * from apples`)
- should not access DiskManager of File directly, 
- should call BufferPool to get page on disk.

example sequence
- tbl_name="apples" is parsed from sql str like select name from apples
- exec_scan calls BTree module to scan specific table name "apples" from query
- maybe BTree returns TableLeafCell, and they are parsed to DataRecord in ExecScan?


TODO
  - [ ] Stage 1: can work with normal table
    - [ ] scan table when table is on 1 page (table `apples` or `oranges` in `sample.db`).
      - [ ] pass test `cli_sql_scan_table_single_page`
    - [ ] scan table that spans multiple pages: interior and table leaf pages.
      - [x] add CellTableInterior (similar to TableLeafCell)
      - [ ] traverse tree with DFS or BFS to return all leaf cells by going thru pointer in interior cell.
      - [ ] pass test `cli_sql_scan_table_multiple_pages`
  - [ ] Stage 2: can work with index table
    - [ ] IndexLeafCell and IndexInteriorCell
    - ???


References
  - [Sqlite dev.to series: Part 3 - Understanding the B-Tree and its role on database design](https://dev.to/thepolyglotprogrammer/what-would-sqlite-look-like-if-written-in-rust-part-3-ool)
  - (optional) Chapter 6, book Subsankar in readings.
  - https://jvns.ca/blog/2014/10/02/how-does-sqlite-work-part-2-btrees/
  - https://saveriomiroddi.github.io/SQLIte-database-file-format-diagrams/


## Buffer Pool
- Buffer Pool is a in-memory cache of pages from the database file on disk.
- All access methods (read and write) MUST go through the buffer pool 
and not the database file directly.


- [ ] `SqliteContextProvider` and `DbMeta`
  when SQLite starts, use buffer pool for parsing db header and metadata from first page.
- [ ] maintain dirty-flag for each page, set if page is modified.
- [ ] `Buffer Replacement Policy` implement LRU policy for page eviction when buffer is full.
  - maintain timestamp of when page was last accessed.
  - when buffer is full, evict page with oldest timestamp.
  - store timestamp in a data structure that allows efficient sorting and retrieving smallest.


## Write Path
TODO


## Concurrency Control
TODO


## Recovery 
- [ ] implement LogRecord
- [ ] LogManager
- [ ] CheckpointManager
- [ ] LogRecovery: reads log file from disk, redo and undo.
