function ColorMyPencils(color)
	color = color or "dracula"
	vim.cmd.colorscheme(color)

	vim.api.nvim_set_hl(0, "Normal", { bg = "none" })
	vim.api.nvim_set_hl(0, "NormalFloat", { bg = "none" })
end

require('feline').setup()

return {
    {"Mofiqul/dracula.nvim", name = "dracula"},
    -- {"LazyVim/LazyVim", opts = {colorscheme = "dracula"},
}