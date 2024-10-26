# git2-rs/libgit2 bug when trying to download AUR repo

```bash
$ rm *-git -rf && cargo build -j $(nproc) && gdb git2-aur
   Compiling git2-aur v0.1.0 (/home/ervin/src/mine/projects/lang/Rust/writing/git2-aur)
    Finished `dev` profile [unoptimized + debuginfo] target(s) in 0.18s
Reading symbols from git2-aur...
warning: Missing auto-load script at offset 0 in section .debug_gdb_scripts
of file /home/ervin/.local/share/cargo/target/debug/git2-aur.
Use `info auto-load python-scripts [REGEXP]' to list them.
(gdb) r
Starting program: /home/ervin/.local/share/cargo/target/debug/git2-aur
[Thread debugging using libthread_db enabled]
Using host libthread_db library "/usr/lib/libthread_db.so.1".

Program received signal SIGSEGV, Segmentation fault.
0x000055555573c348 in ?? ()
(gdb) bt
#0  0x000055555573c348 in ?? ()
#1  0x00007ffff7d38464 in http_parser_execute (parser=parser@entry=0x55555573c348, settings=settings@entry=0x7fffffffa270, data=<optimized out>, len=<optimized out>)
    at /usr/src/debug/http-parser/http-parser-2.9.4/http_parser.c:908
#2  0x00007ffff7f26ff2 in git_http_parser_execute (parser=parser@entry=0x55555573c348, data=<optimized out>, len=<optimized out>)
    at /usr/src/debug/libgit2/libgit2-1.8.2/src/libgit2/transports/httpparser.c:82
#3  0x00007ffff7f28c5e in client_read_and_parse (client=client@entry=0x55555573c320) at /usr/src/debug/libgit2/libgit2-1.8.2/src/libgit2/transports/httpclient.c:1178
#4  0x00007ffff7f291d8 in git_http_client_read_response (response=response@entry=0x7fffffffa420, client=0x55555573c320)
    at /usr/src/debug/libgit2/libgit2-1.8.2/src/libgit2/transports/httpclient.c:1458
#5  0x00007ffff7f304fa in http_stream_read (s=0x555555738840, buffer=0x55555573fc18 "", buffer_size=65536, out_len=0x7fffffffa580)
    at /usr/src/debug/libgit2/libgit2-1.8.2/src/libgit2/transports/http.c:427
#6  0x00007ffff7f329e6 in git_smart__recv (t=t@entry=0x55555573f9f0) at /usr/src/debug/libgit2/libgit2-1.8.2/src/libgit2/transports/smart.c:29
#7  0x00007ffff7f36340 in git_smart__store_refs (t=0x55555573f9f0, flushes=2) at /usr/src/debug/libgit2/libgit2-1.8.2/src/libgit2/transports/smart_protocol.c:58
#8  git_smart__connect (transport=0x55555573f9f0, url=<optimized out>, direction=<optimized out>, connect_opts=<optimized out>)
    at /usr/src/debug/libgit2/libgit2-1.8.2/src/libgit2/transports/smart.c:171
#9  0x00007ffff7eff759 in git_remote_connect_ext (remote=0x55555573b420, direction=GIT_DIRECTION_FETCH, given_opts=<optimized out>)
    at /usr/src/debug/libgit2/libgit2-1.8.2/src/libgit2/remote.c:963
#10 0x00007ffff7e87e67 in clone_into (repo=0x555555728790, _remote=0x55555573d050, opts=0x7fffffffab38, co_opts=0x7fffffffaaa8, branch=<optimized out>)
    at /usr/src/debug/libgit2/libgit2-1.8.2/src/libgit2/clone.c:449
#11 git__clone (out=0x7fffffffc130, url=<optimized out>, local_path=0x55555572f5f0 "./neofetch-git", _options=<optimized out>, use_existing=<optimized out>)
    at /usr/src/debug/libgit2/libgit2-1.8.2/src/libgit2/clone.c:546
#12 0x000055555558a07f in git2::build::RepoBuilder::clone (self=0x7fffffffc798, url=..., into=...) at src/call.rs:13
#13 0x0000555555582ac6 in git2::repo::Repository::clone<&str> (url=..., into=...)
    at /home/ervin/.local/share/cargo/registry/src/index.crates.io-6f17d22bba15001f/git2-0.19.0/src/repo.rs:330
#14 0x000055555558290d in git2_aur::main () at src/main.rs:3
```

First seen in [update-qtile](https://github.com/ervinpopescu/update-qtile).
