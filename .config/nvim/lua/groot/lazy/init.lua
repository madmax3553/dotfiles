return {--print("Lazy/Init"),
{
    "Mofiqul/dracula.nvim",
    transparent = true,
    disable_background = true
}, {
    "LazyVim/LazyVim",
    opts = {
        colorscheme = "dracula"
    }
}, {
    "theprimeagen/harpoon",

    branch = harpoon2,
    dependencies = {"nvim-lua/plenary.nvim"}

    -- config = function()
    --     local harpoon = require("harpoon")

    --     harpoon:setup()

    --     vim.keymap.set("n", "<leader>a", function()
    --         harpoon:list():add()
    --     end)
    --     vim.keymap.set("n", "<C-e>", function()
    --         harpoon.ui:toggle_quick_menu(harpoon:list())
    --     end)

    --     vim.keymap.set("n", "<C-h>", function()
    --         harpoon:list():select(1)
    --     end)
    --     vim.keymap.set("n", "<C-t>", function()
    --         harpoon:list():select(2)
    --     end)
    --     vim.keymap.set("n", "<C-n>", function()
    --         harpoon:list():select(3)
    --     end)
    --     vim.keymap.set("n", "<C-s>", function()
    --         harpoon:list():select(4)
    --     end)
    --     vim.keymap.set("n", "<leader><C-h>", function()
    --         harpoon:list():replace_at(1)
    --     end)
    --     vim.keymap.set("n", "<leader><C-t>", function()
    --         harpoon:list():replace_at(2)
    --     end)
    --     vim.keymap.set("n", "<leader><C-n>", function()
    --         harpoon:list():replace_at(3)
    --     end)
    --     vim.keymap.set("n", "<leader><C-s>", function()
    --         harpoon:list():replace_at(4)
    --     end)
    -- end
}, -- NeoTree
{
    "nvim-neo-tree/neo-tree.nvim",
    branch = "v3.x",
    hidden_files = true,
    dependencies = {"nvim-lua/plenary.nvim", "nvim-tree/nvim-web-devicons", "MunifTanjim/nui.nvim", "3rd/image.nvim"}
}, -- Mason
{
    "williamboman/mason.nvim",
    config = function()
        require("mason").setup()
    end
}, -- TreeSitter
{
    "nvim-treesitter/nvim-treesitter",

    build = ":TSUpdate",

    config = function()
        local configs = require("nvim-treesitter.configs")

        configs.setup({
            ensure_installed = {"c", "lua", "vim", "vimdoc", "html"},
            sync_install = false,
            auto_install = true,
            highlight = {
                enable = true,
                additional_vim_regex_highlighting = {"markdown"}
            },
            indent = {
                enable = true
            }
        })
    end
}, -- Feline 
{
    "famiu/feline.nvim",

    dependencies = {"nvim-tree/nvim-web-devicons", "lewis6991/gitsigns.nvim"},

    config = function()
        require('feline').setup()
    end
}, -- Telescope
{
    "nvim-telescope/telescope.nvim",

    tag = "0.1.8",

    hidden = true,

    dependencies = {"nvim-lua/plenary.nvim"},

    config = function()
        require('telescope').setup({})

        local builtin = require('telescope.builtin')
        vim.keymap.set('n', '<leader>pf', builtin.find_files, {})
        vim.keymap.set('n', '<C-p>', builtin.git_files, {})
        vim.keymap.set('n', '<leader>pws', function()
            local word = vim.fn.expand("<cword>")
            builtin.grep_string({
                search = word
            })
        end)
        vim.keymap.set('n', '<leader>pWs', function()
            local word = vim.fn.expand("<cWORD>")
            builtin.grep_string({
                search = word
            })
        end)
        vim.keymap.set('n', '<leader>ps', function()
            builtin.grep_string({
                search = vim.fn.input("Grep > ")
            })
        end)
        vim.keymap.set('n', '<leader>vh', builtin.help_tags, {})
    end
}, -- Fugitive
{
    "tpope/vim-fugitive",
    config = function()
        vim.keymap.set("n", "<leader>gs", vim.cmd.Git)

        local autocmd = vim.api.nvim_create_autocmd
        autocmd("BufWinEnter", {
            pattern = "*",
            callback = function()
                if vim.bo.ft ~= "fugitive" then
                    return
                end

                local bufnr = vim.api.nvim_get_current_buf()
                local opts = {
                    buffer = bufnr,
                    remap = false
                }
                vim.keymap.set("n", "<leader>p", function()
                    vim.cmd.Git('push')
                end, opts)

                -- rebase always
                vim.keymap.set("n", "<leader>P", function()
                    vim.cmd.Git({'pull', '--rebase'})
                end, opts)

                -- NOTE: It allows me to easily set the branch i am pushing and any tracking
                -- needed if i did not set the branch up correctly
                vim.keymap.set("n", "<leader>t", ":Git push -u origin ", opts);
            end
        })

        vim.keymap.set("n", "gu", "<cmd>diffget //2<CR>")
        vim.keymap.set("n", "gh", "<cmd>diffget //3<CR>")
    end
}}
