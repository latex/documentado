if !exists('g:documentado_bin')
  let g:documentado_bin = 'documentado.exe'
endif

function! documentado#search(query) abort
  " On Windows, open in a new terminal window (cmd.exe /c start)
  " On Unix, use :! to run in the same terminal
  if has('win32')
    call documentado#win_open(a:query)
  elseif has('nvim')
    call documentado#nvim_open(a:query)
  elseif empty(a:query)
    execute '!' . g:documentado_bin
  else
    execute '!' . g:documentado_bin . ' ' . shellescape(a:query)
  endif
endfunction

function! documentado#win_open(query) abort
  let l:cmd = g:documentado_bin
  if !empty(a:query)
    let l:cmd = l:cmd . ' ' . a:query
  endif
  call jobstart(['cmd.exe', '/c', 'start', 'Documentado', '', l:cmd])
endfunction

function! documentado#nvim_open(query) abort
  let l:args = [g:documentado_bin]
  if !empty(a:query)
    call add(l:args, a:query)
  endif

  let l:buf = nvim_create_buf(v:false, v:true)
  call nvim_open_win(l:buf, v:true, {
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
  startinsert
endfunction
