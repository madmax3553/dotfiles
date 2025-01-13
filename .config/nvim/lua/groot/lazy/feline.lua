return {
    {
        "famiu/feline.nvim",
        dependencies = { 'nvim-tree/nvim-web-devicons', 'lewis6991/gitsigns.nvim' },
        config = function()
            require('gitsigns').setup()
            require('feline').setup()
        end
    } }

