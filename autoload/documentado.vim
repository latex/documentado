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

let s:doc_win = -1

function! documentado#nvim_open(query) abort
  let l:cmd = g:documentado_bin
  if !empty(a:query)
    let l:cmd = l:cmd . ' ' . shellescape(a:query)
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

  call termopen(l:cmd, {'on_exit': function('documentado#on_exit')})
  setlocal bufhidden=wipe
  startinsert
endfunction

function! documentado#on_exit(...) abort
  if s:doc_win > 0 && nvim_win_is_valid(s:doc_win)
    call nvim_win_close(s:doc_win, v:true)
  endif
  let s:doc_win = -1
endfunction
