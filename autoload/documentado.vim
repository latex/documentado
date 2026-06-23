if !exists('g:documentado_bin')
  let g:documentado_bin = 'documentado.exe'
endif

function! documentado#search(query) abort
  if empty(a:query)
    execute '!' . g:documentado_bin
  else
    execute '!' . g:documentado_bin . ' ' . shellescape(a:query)
  endif
endfunction
