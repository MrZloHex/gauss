" Vim syntax file
" Language: Gauss

" Usage Instructions
" Put this file in .vim/syntax/gauss.vim
" and add in your .vimrc file the next line:
" autocmd BufRead,BufNewFile *.gis set filetype=gauss
" autocmd BufRead,BufNewFile *.gfs set filetype=gauss


if exists("b:current_syntax")
  finish
endif

" Language keywords
syntax keyword gaussKeywords RET SYSCALL UNRET
syntax keyword gaussConditionKeywords IF THEN
syntax keyword gaussLoopKeywords WHILE DO FOR LOOP
syntax keyword gaussDirectiveKeywords USES SET
syntax keyword gaussTypeKeywords BYTE WORD DWORD QWORD PTR SEQ NULL

" Values
syntax region gaussImmediateValue start="#" end="\d\+"

" Comments
syntax keyword gaussTodo contained TODO FIXME XXX NOTE
syntax region gaussCommentLine start=";" end="$" contains=gaussTodo

" Set highlights
highlight default link gaussTypeKeywords Type
highlight default link gaussKeywords Statement
highlight default link gaussConditionKeywords Conditional
highlight default link gaussLoopKeywords Repeat
highlight default link gaussDirectiveKeywords Include
highlight default link gaussImmediateValue Number
highlight default link gaussCommentLine Comment
highlight default link gaussTodo Todo


let b:current_syntax = "gauss"
