return {
	{
		"nvim-neorg/neorg",
		lazy = false,
		version = "*",
		config = function()
			require("neorg").setup({
				load = {
					["core.defaults"] = {},
					["core.concealer"] = {},
					["core.summary"] = {},
					["core.dirman"] = {
						config = {
							workspaces = {
								notes = "~/journal/notes",
								tasks = "~/journal/tasks",
								projects = "~/journal/projects",
								ideas = "~/journal/ideas",
								plans = "~/journal/plans",
							},
							default_workspace = "notes",
						},
					},
				},
			})
			vim.wo.foldlevel = 99
			vim.wo.conceallevel = 2
		end,
		dependencies = { "nvim-treesitter", "nvim-lua/plenary.nvim", "nvim-neorg/neorg-telescope" },
	},
}
