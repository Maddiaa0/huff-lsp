# Huff lsp

## Introduction

A language server for Huff. I've got some free time so why not

## Development

1. `pnpm i`
2. `cargo build`
3. press <kbd>F5</kbd> or change to the Debug panel and click <kbd>Launch Client</kbd>

## Features

- [x] InlayHint for LiteralType
      ![inlay hint](https://user-images.githubusercontent.com/17974631/156926412-c3823dac-664e-430e-96c1-c003a86eabb2.gif)

- [x] semantic token  
       make sure your semantic token is enabled, you could enable your `semantic token` by
      adding this line to your `settings.json`

```json
{
  "editor.semanticHighlighting.enabled": true
}
```

## TODO:

- [x] Code Completion
- [ ] Macro Renaming
- [ ] Fault tolerant syntactic error diagnostics
- [ ] Go to definition - for nested file types
- [ ] Find Reference
- [ ] Stack Assertions
- [ ] Macro Level Debugging
- [ ] Static analysis, underflow / overflow
- [ ] Stack Comment Generations
