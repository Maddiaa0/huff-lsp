# Huff lsp

## Introduction

A language server for Huff. (finally)

## Development

1. `pnpm i`
2. `cargo build`
3. press <kbd>F5</kbd> or change to the Debug panel and click <kbd>Launch Client</kbd>

## Features

```json
{
  "editor.semanticHighlighting.enabled": true
}
```

## TODO:

- [x] Code Completion
- [ ] InlayHint for stack height at current line

- [ ] semantic token  
       make sure your semantic token is enabled, you could enable your `semantic token` by
      adding this line to your `settings.json`

- [ ] Macro Renaming
- [ ] Fault tolerant syntactic error diagnostics
- [ ] Go to definition - for nested file types
- [ ] Find Reference
- [ ] Stack Assertions
- [ ] Macro Level Debugging
- [ ] Static analysis, underflow / overflow
- [ ] Stack Comment Generations
