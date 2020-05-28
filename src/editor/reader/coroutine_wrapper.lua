-- This function wraps all functions of the given table in coroutine yields
-- We use this since this is not doable from the rlua API
return function(internal_functions)
    local lm = {}

    for name, func in pairs(internal_functions) do
        lm[name] = function(...)
            return coroutine.yield(func(...))
        end
    end

    return lm
end