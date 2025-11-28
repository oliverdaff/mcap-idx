# mcap-stream

A small, exploratory Rust parser for MCAP files with a focus on **forward-only, streaming reads**. I’m using this project to experiment with ways of indexing large MCAP recordings without relying on mmap or random access, and to get a deeper feel for the format from the ground up.

It’s early days, but the goals are straightforward:

- Parse MCAP records from an async `Read` without seeking.
- Build lightweight in-memory indexes as we stream.
- Explore structures that stay fast on very large files (PGM, learned indexes, rank/select, segment-style layouts).
- Keep the code simple, well-tested, and easy to extend.

Right now it can step through basic record types and surface their headers. More features will land as I refine the parser and flesh out a proper indexing path.

If you’re curious, feel free to watch the repo — I’ll keep improving it in the open.
