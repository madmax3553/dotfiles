return{
    'stevearc/conform.nvim',
    opts = {
        formatters_by_ft = {
            lua = { 'stylua' },
            javascript = { 'prettier' },
            typescript = { 'prettier' },
            html = { 'prettier' },
            css = { 'prettier' },
            json = { 'prettier' },
            markdown = { 'prettier' },
            yaml = { 'prettier' },
            sh = { 'shfmt' },
            go = { 'gofmt' },
            rust = { 'rustfmt' },
        },
        format_on_save = {
            timeout_ms  = 500,
            lsp_format = "fallback",
        },
    },
}
