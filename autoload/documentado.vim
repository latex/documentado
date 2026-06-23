if !exists('g:documentado_bin')
  let g:documentado_bin = 'documentado.exe'
endif

function! documentado#search(query) abort
  if has('nvim')
    call documentado#nvim_open(a:query)
  elseif empty(a:query)
    execute '!' . g:documentado_bin
  else
    execute '!' . g:documentado_bin . ' ' . shellescape(a:query)
  endif
endfunction

function! documentado#nvim_open(query) abort
  let l:args = [g:documentado_bin]
  if !empty(a:query)
    call add(l:args, a:query)
  endif

  let l:buf = nvim_create_buf(v:false, v:true)
  let s:doc_win = nvim_open_win(l:buf, v:true, {
        \ 'relative': 'editor',
        \ 'width': max([80, &columns - 8]),
        \ 'height': max([24, &lines - 6]),
        \ 'col': 4,
        \ 'row': 2,
        \ 'style': 'minimal',
        \ 'border': 'rounded',
        \ 'title': ' Documentado ',
        \ })

  call termopen(l:args)
  setlocal bufhidden=wipe
  tnoremap <buffer> <C-q> <C-\><C-n>:close<CR>
  startinsert
endfunction
