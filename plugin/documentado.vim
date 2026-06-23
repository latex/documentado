if exists('g:loaded_documentado')
  finish
endif
let g:loaded_documentado = 1

command! -nargs=? -range=0 Documentado call documentado#search(<q-args>)

nnoremap <silent> <leader>d :call documentado#search(expand('<cword>'))<CR>
xnoremap <silent> <leader>d y:call documentado#search(@")<CR>
