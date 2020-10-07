nmap <leader>r :silent !RUN_TERMINAL=1 bin/run<CR>
nmap <leader>b :silent !RUN_TERMINAL=1 CARGO_CMD=build bin/run<CR>

autocmd BufNewFile,BufRead *.md        set      syntax=markdown filetype=markdown
autocmd BufNewFile,BufRead *.tsx,*.tmx setlocal syntax=xml filetype=xml
autocmd BufNewFile,BufRead *.yml       setlocal shiftwidth=2 softtabstop=2 tabstop=2
