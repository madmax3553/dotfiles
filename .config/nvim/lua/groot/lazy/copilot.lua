return{
   "github/copilot.vim",
    event = { "BufReadPost", "BufNewFile" }, -- Lazy load on buffer read/new file
    config = function()
      -- Optional: Add any configuration for copilot.vim here
      -- vim.g.copilot_no_tab_map = true  -- Prevents copilot from mapping tab by default
    end,
}
