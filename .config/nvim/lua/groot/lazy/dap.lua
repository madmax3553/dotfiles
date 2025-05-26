return {
	{
		'mfussenegger/nvim-dap',
		lazy = true,
		config = function()
			require('dap').defaults.fallback.external_terminal = {
				command = 'usr/bin/ghostty',
			}
		end,
		keys = {
			{ '<leader>db', function() require('dap').toggle_breakpoint() end, desc = 'Toggle Breakpoint' },
			{ '<leader>dc', function() require('dap').continue() end, desc = 'Continue' },
			{ '<leader>dn', function() require('dap').step_over() end, desc = 'Step Over' },
			{ '<leader>di', function() require('dap').step_into() end, desc = 'Step Into' },
			{ '<leader>do', function() require('dap').step_out() end, desc = 'Step Out' },
			{ '<leader>dt', function() require('dap').terminate() end, desc = 'Terminate' },
		},
	},
	-- DAP UI for better debugging experience
	{
		'rcarriga/nvim-dap-ui',
		dependencies = { 'mfussenegger/nvim-dap', 'nvim-neotest/nvim-nio' },
		config = function()
			require('dapui').setup({
				layouts = {
					{
						elements = {
							{ id = 'scopes', size = 0.35 },
							{ id = 'breakpoints', size = 0.30 },
							{ id = 'repl', size = 0.35 },
						},
						position = 'right',
						size = 50,
					},
				},
				controls = {
					element = 'scopes',
					enabled = true,
				},
			})
			local dap, dapui = require('dap'), require('dapui')
			dap.listeners.before.attach.dapui_config = function() dapui.open() end
			dap.listeners.before.launch.dapui_config = function() dapui.open() end
			dap.listeners.before.event_terminated.dapui_config = function() dapui.close() end
			dap.listeners.before.event_exited.dapui_config = function() dapui.close() end
		end,
	},
	-- Go-specific DAP extension
	{
		'leoluz/nvim-dap-go',
		ft = 'go',
		dependencies = { 'mfussenegger/nvim-dap' },
		config = function()
			require('dap').configurations.go = {}
			require('dap-go').setup({
				delve = {
					path = 'dlv', -- Ensure Delve is in $PATH
					initialize_timeout_sec = 20,
					port = '${port}', -- Random port
					detached = false, -- Critical for Windows, harmless on Linux
				},
				dap_configurations = {
					{
						type = 'go',
						name = 'Debug (External Terminal)',
						request = 'launch',
						mode = 'debug',
						program = '${fileDirname}',
						console = 'externalTerminal',
					},
					{
						type = 'go',
						name = 'Debug Package',
						request = 'launch',
						mode = 'debug',
						program = '${fileDirname}',
					},
					{
						type = 'go',
						name = 'Debug File',
						request = 'launch',
						mode = 'debug',
						program = '${file}',
					},
				},
			})
		end,
		keys = {
			{ '<leader>dT', function() require('dap-go').debug_test() end, desc = 'Debug Nearest Test' },
			{
				'<leader>dP',
				function() require('dap').run({ type = 'go', name = 'Debug Package' }) end,
				desc = 'Debug Package',
			},
			{
				'<leader>dF',
				function() require('dap').run({ type = 'go', name = 'Debug File' }) end,
				desc = 'Debug File',
			},
		},
	},
	-- Optional: Persistent breakpoints
	{
		'Weissle/persistent-breakpoints.nvim',
		dependencies = { 'mfussenegger/nvim-dap' },
		config = function()
			require('persistent-breakpoints').setup({
				load_breakpoints_event = { 'BufReadPost' },
			})
		end,
	},
	-- Optional: Virtual text for variable values
	{
		'theHamsta/nvim-dap-virtual-text',
		dependencies = { 'mfussenegger/nvim-dap' },
		config = function() require('nvim-dap-virtual-text').setup({}) end,
	},
}
