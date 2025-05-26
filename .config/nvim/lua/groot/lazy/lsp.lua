return {
	'neovim/nvim-lspconfig',
	dependencies = {
		'saghen/blink.cmp',
		'williamboman/mason.nvim',
		'williamboman/mason-lspconfig.nvim',
		'WhoIsSethDaniel/mason-tool-installer.nvim',
		'j-hui/fidget.nvim',
	},

	-- example using `opts` for defining servers
	opts = {
		servers = {
			lua_ls = {},
		},
	},
	config = function(_, opts)
		local lspconfig = require('lspconfig')
		for server, config in pairs(opts.servers) do
			config.capabilities = require('blink.cmp').get_lsp_capabilities(config.capabilities)
			lspconfig[server].setup(config)
		end
		require('fidget').setup({})
		require('mason').setup()
		require('mason-lspconfig').setup({
			ensure_installed = {
				'lua_ls',
				'rust_analyzer',
				'gopls',
			},
			handlers = {
				function(server_name) -- default handler (optional)
					require('lspconfig')[server_name].setup({
						capabilities = capabilities,
					})
				end,

				zls = function()
					local lspconfig = require('lspconfig')
					lspconfig.zls.setup({
						root_dir = lspconfig.util.root_pattern('.git', 'build.zig', 'zls.json'),
						settings = {
							zls = {
								enable_inlay_hints = true,
								enable_snippets = true,
								warn_style = true,
							},
						},
					})
					vim.g.zig_fmt_parse_errors = 0
					vim.g.zig_fmt_autosave = 0
				end,
				['lua_ls'] = function()
					local lspconfig = require('lspconfig')
					lspconfig.lua_ls.setup({
						capabilities = capabilities,
						settings = {
							Lua = {
								runtime = { version = 'Lua 5.1' },
								diagnostics = {
									globals = { 'bit', 'vim', 'it', 'describe', 'before_each', 'after_each' },
								},
							},
						},
					})
				end,
			},
		})
	end,
}
