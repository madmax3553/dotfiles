return {
    "lukas-reineke/indent-blankline.nvim",
    main = "ibl",
    ---@module "ibl"
    ---@type ibl.config
    opts = {
        exclude = {
            buftypes = {
                "terminal",
                "nofile",
            },
            filetypes = {
                "help",
                "lazy",
                "mason",
                "NvimTree",
                "Outline",
                "toggleterm",
                "Trouble",
                "TelescopePrompt",
                "lspinfo",
                "dashboard",
            },
        },
    },
}
