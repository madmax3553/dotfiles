-- This file can be loaded by calling `lua require('plugins')` from your init.vim

-- Only required if you have packer configured as `opt`
vim.cmd [[packadd packer.nvim]]

return require('packer').startup(function(use)
  -- Packer can manage itself
  use 'wbthomason/packer.nvim'
  use { 'catppuccin/nvim', as = 'catppuccin' }
  use { 'nvim-telescope/telescope.nvim',
    tag = '0.1.6',
    config = function()
        require'telescope'.setup({
            defaults = {
                vimgrep_arguments = {
                    'rg',
                    '--hidden',
                },
            hidden = true
            },
        })
    end,
    requires = { {'nvim-lua/plenary.nvim'} },
    -- requires('telescope').load_extension('fzy_native')
  }
  use 'feline-nvim/feline.nvim'
  use 'lewis6991/gitsigns.nvim'
  use 'nvim-tree/nvim-web-devicons'
  use { 'nvim-treesitter/nvim-treesitter',
    run = ':TSUpdate<cr>'
    }
  use 'nvim-treesitter/playground'
  use 'theprimeagen/harpoon'
  use 'mbbill/undotree'
  use 'tpope/vim-fugitive'
  use 'junegunn/fzf'
  use {'nvimtools/none-ls.nvim',
  	config = function()
		local null_ls = require("null-ls")
		null_ls.setup({
			sources = {
				null_ls.builtins.formatting.stylua,
			}
		})
	end,
}
use {
  "nvim-neorg/neorg",
  rocks = { "lua-utils.nvim", "nvim-nio", "nui.nvim", "plenary.nvim", "pathlib.nvim" },
  tag = "*", -- Pin Neorg to the latest stable release
  config = function()
      require("neorg").setup()
  end,
}
  use { 'nvimdev/dashboard-nvim',
  event = 'VimEnter',
  config = function()
      require('dashboard').setup{
          theme = 'hyper',
          config = {
              week_header = {
                  enable = true,
              },
              shortcut = {
                  { desc = '󰊳 Update', group = '@property', action = 'Packer', key = 'u' },
                  {
                      icon = ' ',
                      icon_hl = '@variable',
                      desc = 'Files',
                      group = 'Label',
                      action = 'Telescope find_files',
                      key = 'f',
                  },
                  {
                      desc = ' Apps',
                      group = 'DiagnosticHint',
                      action = 'Telescope app',
                      key = 'a',
                  },
                  {
                      desc = ' dotfiles',
                      group = 'Number',
                      action = 'Telescope dotfiles',
                      key = 'd',
                  },
              },
          },
      }
  end,
  requires = { {'nvimtree/nvim-web-devicons'} },
  }
  use { 'VonHeikemen/lsp-zero.nvim',
    branch = 'v3.x',
    requires = {
      {'williamboman/mason.nvim'},
      {'williamboman/mason-lspconfig.nvim'},
      {'neovim/nvim-lspconfig'},
      {'hrsh7th/nvim-cmp'},
      {'hrsh7th/cmp-nvim-lsp'},
      {'L3MON4D3/LuaSnip'},
    }
  }
  use { 'nvim-neo-tree/neo-tree.nvim',
    branch = "v3.x",
    requires = {
      {'nvim-lua/plenary.nvim'},
      {'nvim-tree/nvim-web-devicons'},
      {'MunifTanjim/nui.nvim'},
      {'3rd/image.nvim'},
      {'s1n7ax/nvim-window-picker',
        version = '2.*',
        config = function()
            require 'window-picker'.setup({
                filter_rules = {
                  include_current_win = false,
                  autoselect_one = true,
                  bo = {
                      filetype = { 'neo-tree', "neo-tree-popup", "notify" },
                      buftype = { 'terminal', "quickfix" },
                  },
              },
          })
      end,
  },
      },
  }
end)
