return {
	--"github/copilot.vim",
	-- event = { "BufReadPost", "BufNewFile" }, -- Lazy load on buffer read/new file
	-- config = function()
	--   -- Optional: Add any configuration for copilot.vim here
	--   -- vim.g.copilot_no_tab_map = true  -- Prevents copilot from mapping tab by default
	-- end,
	{
		'zbirenbaum/copilot.lua',
		cmd = 'Copilot',
		event = { 'BufReadPost', 'BufNewFile' },
		config = function()
			require('copilot').setup({
				suggestion = {
					enabled = true,
					auto_trigger = true,
					keymap = {
						accept = '<C-l>', -- Replace with your preferred key
						next = '<C-j>',
						prev = '<C-k>',
						dismiss = '<C-h>',
					},
				},
				filetypes = {
					['*'] = true, -- Enable for all filetypes, adjust as needed
					go = true, -- Explicitly enable for Go
				},
				server_opts_overrides = {
					settings = {
						advanced = {
							inlineSuggest = true, -- Inline suggestions for Go
						},
					},
				},
			})
		end,
	},
	{
		'CopilotC-Nvim/CopilotChat.nvim',
		branch = 'main',
		dependencies = {
			{ 'zbirenbaum/copilot.lua' },
			{ 'nvim-lua/plenary.nvim' },
		},
		config = function()
			require('CopilotChat').setup({
				window = {
					layout = 'float', -- Floating window for chat
					width = 0.8,
					height = 0.8,
				},
			})
		end,
	},
}
